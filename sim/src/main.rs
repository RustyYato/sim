use std::sync::Arc;

use bevy::{app::AppExit, prelude::*};
use rand::{prelude::SmallRng, Rng, SeedableRng};

mod args;

fn main() {
    let mut args = args::Args::parse();
    args.seed = args.raw_seed.unwrap_or_else(rand::random);
    let args = Arc::new(args);

    eprintln!("Seed = {}", args.seed);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system({
            let args = args.clone();
            move |commands: Commands, window: Res<Windows>, assets: ResMut<Assets<Image>>| {
                startup(&args, commands, window, assets)
            }
        })
        .add_system(exit_on_escape_key)
        .add_system(move_all)
        .run();
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn exit_on_escape_key(input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

#[derive(Component, Clone, Copy)]
struct CritterType;

#[derive(Bundle)]
pub struct Critter {
    critter_type: CritterType,
    #[bundle]
    sprite: SpriteBundle,
    velocity: Velocity,
}

#[derive(Component, Clone, Copy)]
struct Velocity(Vec2);

fn circle() -> Image {
    let size = 100;
    let falloff = 3;
    let half = size / 2;
    let mut data = vec![0; (size * size) as usize * 4];

    for i in 0..size {
        for j in 0..size {
            let x = i - half;
            let y = j - half;

            if x * x + y * y < half * half {
                let dist = f64::from(x * x + y * y).sqrt();
                let falloff = (dist - f64::from(half - falloff)) / f64::from(falloff);
                let falloff = 1.0 - falloff.max(0.0);

                let index = (i * size + j) as usize;
                let data = &mut data[index * 4..][..4];
                data.fill((f64::from(0xff) * falloff) as u8);
            }
        }
    }

    let size = size as u32;

    Image::new(
        bevy::render::render_resource::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
    )
}

fn startup(
    args: &args::Args,
    mut commands: Commands,
    window: Res<Windows>,
    mut assets: ResMut<Assets<Image>>,
) {
    let mut rng = SmallRng::seed_from_u64(args.seed);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = window.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let circle = assets.add(circle());

    for _ in 0..args.critters {
        let mut rng = SmallRng::from_seed(rng.gen());
        commands.spawn().insert_bundle(Critter {
            critter_type: CritterType,
            velocity: Velocity({
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                Vec2::new(angle.cos(), angle.sin()) * rng.gen_range(10.0..40.0)
            }),
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::from_slice(&{
                        let mut rng = SmallRng::from_seed(rng.gen());
                        [
                            rng.gen_range(-width / 4.0..width / 4.0),
                            rng.gen_range(-height / 4.0..height / 4.0),
                            0.0,
                        ]
                    }),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, rng.gen_range(0.2..0.8)),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..Default::default()
                },
                texture: circle.clone(),
                ..Default::default()
            },
        });
    }
}

fn move_all(
    time: Res<Time>,
    mut creatures: Query<(&mut Transform, &Velocity)>,
    window: Res<Windows>,
) {
    let window = window.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let time = time.delta_seconds();

    creatures.for_each_mut(|(mut transform, Velocity(vel))| {
        let transform: &mut Transform = &mut *transform;
        transform.compute_matrix();
        transform.translation.x += vel.x * time;
        transform.translation.y += vel.y * time;

        if transform.translation.x > width / 2.0 {
            transform.translation.x -= width;
        } else if transform.translation.x < -width / 2.0 {
            transform.translation.x += width;
        }
        if transform.translation.y > height / 2.0 {
            transform.translation.y -= height;
        } else if transform.translation.y < -height / 2.0 {
            transform.translation.y += height;
        }
    })
}
