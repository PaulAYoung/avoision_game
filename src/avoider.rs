use bevy::{prelude::*, render::renderer::RenderContext};
use crate::components::{
    Position
};
use crate::constants::AVOIDER_SPEED;
use crate::materials::Materials;
pub struct Avoider;

pub fn spawn_avoider(
    mut commands: Commands,
    materials: Res<Materials>,
){
    commands.spawn()
        .insert_bundle(
        SpriteBundle {
            material: materials.avoidee_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        }
    )
    .insert(Position(Vec2::new(0.0, 0.0)))
    .insert(Avoider);
}

pub fn avoidee_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut avoidee_positions: Query<&mut Position, With<Avoider>>,
    time: Res<Time>
){
    let move_dist = AVOIDER_SPEED * time.delta_seconds();
    for mut pos in avoidee_positions.iter_mut(){
        if keyboard_input.pressed(KeyCode::Up){
            pos.0.y += move_dist;
        } else if keyboard_input.pressed(KeyCode::Down){
            pos.0.y -= move_dist;
        }  else if keyboard_input.pressed(KeyCode::Left){
            pos.0.x -= move_dist;
        }  else if keyboard_input.pressed(KeyCode::Right){
            pos.0.x += move_dist;
        }
    }
}