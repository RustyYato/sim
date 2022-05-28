use bevy::{app::AppExit, prelude::*};
use bevy_rapier2d::prelude::*;

mod args;

mod tools;

mod critters;
mod food;

#[derive(Component, Debug, Clone, Copy)]
enum EntityType {
    Critter,
    Food,
}

fn main() {
    let mut args = args::Args::parse();
    args.seed = args.raw_seed.unwrap_or_else(rand::random);

    eprintln!("Seed = {}", args.seed);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(args)
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_rapier2d::plugin::RapierPhysicsPlugin::<
            bevy_rapier2d::plugin::NoUserData,
        >::with_physics_scale(1.0))
        .add_startup_system(startup)
        .add_startup_system(critters::startup)
        .add_startup_system(food::startup)
        .add_system(exit_on_escape_key)
        .add_system(move_all)
        .add_system(no_food_means_dead)
        .add_system_to_stage(CoreStage::PostUpdate, eat_food_particle)
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

    creatures.for_each_mut(|(mut transform, vel, mut health)| {
        let transform: &mut Transform = &mut *transform;
        let health: Option<&mut Health> = health.as_deref_mut();

        let vel: &Velocity = vel;
        let vel = vel.linvel;

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
        .filter(|(_, &Health(health))| health <= 0.0)
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn eat_food_particle(
    mut collision_events: EventReader<bevy_rapier2d::pipeline::CollisionEvent>,
    mut entities: Query<(&mut Health, &EntityType)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            &CollisionEvent::Started(a, b, flags) => match entities.get_many_mut([a, b]) {
                Ok([(critter_health, EntityType::Critter), (food_health, EntityType::Food)])
                | Ok([(food_health, EntityType::Food), (critter_health, EntityType::Critter)]) => {
                    let mut critter_health = critter_health;
                    let mut food_health = food_health;
                    critter_health.0 += core::mem::take(&mut food_health.0);
                }
                Ok(_) => (),
                Err(err) => (),
            },
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}
