use std::sync::Arc;

use bevy::{app::AppExit, prelude::*};

mod args;

mod tools;

mod critters;

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
                critters::startup(&args, commands, window, assets)
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
struct Velocity(Vec2);

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
