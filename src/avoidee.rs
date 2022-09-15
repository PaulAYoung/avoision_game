use bevy::prelude::*;

use crate::game_structs::GameEntity;
use crate::gep::{Position, Momentum, Collider};
use crate::constants::{ITEM_SIZE, POSITION_SCALE};
use crate::materials::Materials;

#[derive(Component, Default)]
pub struct Avoidee;

#[derive(Bundle, Default)]
struct AvoideeBundle{
    position: Position,
    momentum: Momentum,
    avoidee_marker: Avoidee,
    collider: Collider,
    game_entity: GameEntity,

    #[bundle]
    sprite: SpriteBundle
}
pub struct AvoideeSpawnEvent{
    pub position: Position,
    pub momentum: Momentum
}

pub fn spawn_avoidee_event_reader(
    mut commands: Commands,
    materials: Res<Materials>,
    mut spawn_events: EventReader<AvoideeSpawnEvent>
){
    for e in spawn_events.iter(){
        spawn_avoidee(&mut commands, &materials, e.position, e.momentum);
    }
}


pub fn spawn_avoidee(
    commands: &mut Commands,
    materials: &Res<Materials>,
    position: Position,
    momentum: Momentum
){
    commands.spawn()
        .insert_bundle(
        AvoideeBundle {
            position: position,
            momentum: momentum,
            collider: Collider::Circle{radius: ITEM_SIZE/2.0},
            avoidee_marker: Avoidee,
            sprite: SpriteBundle {
                sprite: Sprite{
                    color: Color::rgb(0.0, 0.7, 0.7),
                    custom_size: Some(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                    ..Default::default()
                },
                transform: Transform{
                    translation: (position.0*POSITION_SCALE).extend(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    );
}