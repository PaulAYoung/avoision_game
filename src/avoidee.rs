use bevy::{prelude::*, render::renderer::RenderContext};
use crate::components::{
    Position
};
use crate::constants::AVOIDEE_SPEED;
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

pub fn avoidee_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut avoidee_positions: Query<&mut Position, With<Avoidee>>,
    time: Res<Time>
){
    let move_dist = AVOIDEE_SPEED * time.delta_seconds();
    for mut pos in avoidee_positions.iter_mut(){
        if keyboard_input.pressed(KeyCode::Up){
            pos.y += move_dist;
        } else if keyboard_input.pressed(KeyCode::Down){
            pos.y -= move_dist;
        }  else if keyboard_input.pressed(KeyCode::Left){
            pos.x -= move_dist;
        }  else if keyboard_input.pressed(KeyCode::Right){
            pos.x += move_dist;
        }
    }
}