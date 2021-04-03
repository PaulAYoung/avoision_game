use bevy::prelude::*;
use crate::components::{
    Position
};
use crate::materials::Materials;


pub struct Avoidee;

pub fn spawn_avoidee(
    commands: &mut Commands,
    materials: Res<Materials>,
){
    commands.spawn(
        SpriteBundle {
            material: materials.avoidee_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        }
    )
    .with(Position{x:0.0, y: 0.0})
    .with(Avoidee);
}