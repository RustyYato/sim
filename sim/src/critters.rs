use bevy::prelude::*;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Sensor;
use rand::{prelude::SmallRng, Rng, SeedableRng};

use bevy_rapier2d::dynamics::{RigidBody, Velocity};
use bevy_rapier2d::geometry::{ActiveCollisionTypes, Collider};

use crate::EntityType;
use crate::{args::Args, tools, Health};

#[derive(Component, Clone, Copy)]
pub struct CritterType;

#[derive(Bundle)]
pub struct Critter {
    critter_type: CritterType,
    entity_type: EntityType,
    #[bundle]
    sprite: SpriteBundle,
    velocity: Velocity,
    health: Health,

    collider: Collider,
    active_collision_types: ActiveCollisionTypes,
    rigid_body: RigidBody,
    sensor: Sensor,
    events: ActiveEvents,
}

pub fn startup(
    args: Res<Args>,
    mut commands: Commands,
    window: Res<Windows>,
    mut assets: ResMut<Assets<Image>>,
) {
    let mut rng = SmallRng::seed_from_u64(args.seed);

    let window = window.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let texture = assets.add(tools::oblique_circle());

    for _ in 0..args.critters {
        let mut rng = SmallRng::from_seed(rng.gen());
        commands.spawn().insert_bundle(Critter {
            critter_type: CritterType,
            entity_type: EntityType::Critter,
            health: Health(
                if (args.health.init_max - args.health.init_min).abs() < 0.01 {
                    args.health.init_max
                } else {
                    rng.gen_range(args.health.init_min..args.health.init_max)
                },
            ),
            velocity: Velocity::linear({
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                Vec2::new(angle.cos(), angle.sin()) * rng.gen_range(10.0..100.0)
            }),
            sprite: SpriteBundle {
                transform: Transform::from_xyz(
                    rng.gen_range(-width / 4.0..width / 4.0),
                    rng.gen_range(-height / 4.0..height / 4.0),
                    0.0,
                ),
                sprite: Sprite {
                    color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, rng.gen_range(0.2..0.8)),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..Default::default()
                },
                texture: texture.clone(),
                ..Default::default()
            },
            collider: Collider::ball(10.0),
            rigid_body: RigidBody::KinematicVelocityBased,
            active_collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            sensor: Sensor(true),
            events: ActiveEvents::COLLISION_EVENTS,
        });
    }
}
