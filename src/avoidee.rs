use rand::prelude::random;
use bevy::prelude::*;

use crate::components::{Position, Momentum};
use crate::materials::Materials;
use crate::constants::{MAX_SPEED};

struct Avoidee;

pub fn spawn_avoidee(
    mut commands: Commands,
    materials: Res<Materials>,
){
    println!("In spawn avoidee!");
    commands.spawn()
        .insert_bundle(
        SpriteBundle {
            material: materials.avoidee_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        }
    )
    .insert(Position(
        Vec2::new(10.0, 10.0)
        //Vec2::new(random::<f32>() * (ARENA_WIDTH as f32), random::<f32>() * (ARENA_HEIGHT as f32))
    ))
    .insert(Momentum(Vec2::new(
            random::<f32>()*random::<f32>()*random::<f32>(),
            random::<f32>()*random::<f32>()*random::<f32>()
        )*MAX_SPEED)
    )
    .insert(Avoidee);
}