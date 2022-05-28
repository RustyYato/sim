use bevy::prelude::*;
use rand::{prelude::SmallRng, Rng, SeedableRng};

use crate::{args::Args, tools, Health};

#[derive(Component, Clone, Copy)]
struct FoodType;

#[derive(Bundle)]
pub struct Food {
    food_type: FoodType,
    #[bundle]
    sprite: SpriteBundle,
    health: Health,
}

pub fn startup(
    mut commands: Commands,
    args: Res<Args>,
    window: Res<Windows>,
    mut assets: ResMut<Assets<Image>>,
) {
    let mut rng = SmallRng::seed_from_u64(args.seed);

    let window = window.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let texture = assets.add(tools::circle());

    for _ in 0..args.food.count {
        let mut rng = SmallRng::from_seed(rng.gen());

        commands.spawn().insert_bundle(Food {
            food_type: FoodType,
            health: Health(if (args.food.init_max - args.food.init_min).abs() < 0.01 {
                args.food.init_max
            } else {
                rng.gen_range(args.food.init_min..args.food.init_max)
            }),
            sprite: SpriteBundle {
                transform: Transform::from_xyz(
                    rng.gen_range(-width / 2.0..width / 2.0),
                    rng.gen_range(-height / 2.0..height / 2.0),
                    0.0,
                ),
                sprite: Sprite {
                    color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, rng.gen_range(0.2..0.8)),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..Default::default()
                },
                texture: texture.clone(),
                ..Default::default()
            },
        });
    }
}
