use std::sync::Arc;

use bevy::{app::AppExit, prelude::*};

mod args;

mod tools;

mod critters;
mod food;

fn main() {
    let mut args = args::Args::parse();
    args.seed = args.raw_seed.unwrap_or_else(rand::random);

    eprintln!("Seed = {}", args.seed);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(args)
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_startup_system(critters::startup)
        .add_startup_system(food::startup)
        .add_system(exit_on_escape_key)
        .add_system(move_all)
        .add_system(no_food_means_dead)
        .run();
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn exit_on_escape_key(input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component, Clone, Copy)]
struct Velocity(Vec2);

#[derive(Component, Clone, Copy)]
struct Health(f32);

fn move_all(
    time: Res<Time>,
    window: Res<Windows>,
    args: Res<args::Args>,
    mut creatures: Query<(&mut Transform, &Velocity, Option<&mut Health>)>,
) {
    let window = window.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let time = time.delta_seconds();

    creatures.for_each_mut(|(mut transform, Velocity(vel), mut health)| {
        let transform: &mut Transform = &mut *transform;
        let health: Option<&mut Health> = health.as_deref_mut();

        transform.compute_matrix();
        transform.translation.x += vel.x * time;
        transform.translation.y += vel.y * time;

        let angle = f32::atan2(vel.y, vel.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);

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

        if let Some(health) = health {
            health.0 -= vel.length() * time * args.health.per_vel;
        }
    })
}

fn no_food_means_dead(mut commands: Commands, creatures: Query<(Entity, &Health)>) {
    creatures
        .iter()
        .filter(|(_, &Health(health))| health < 0.0)
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}
