use bevy::prelude::*;
use rand::{prelude::SmallRng, Rng, SeedableRng};

use crate::{args::Args, tools, Velocity};

#[derive(Component, Clone, Copy)]
struct CritterType;

#[derive(Bundle)]
pub struct Critter {
    critter_type: CritterType,
    #[bundle]
    sprite: SpriteBundle,
    velocity: Velocity,
}

pub fn startup(
    args: &Args,
    mut commands: Commands,
    window: Res<Windows>,
    mut assets: ResMut<Assets<Image>>,
) {
    let mut rng = SmallRng::seed_from_u64(args.seed);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = window.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let circle = assets.add(tools::oblique_circle());

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
