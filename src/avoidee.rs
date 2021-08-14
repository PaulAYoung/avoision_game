use bevy::prelude::*;

use crate::gep::{Position, Momentum};
use crate::constants::ITEM_SIZE;
use crate::materials::Materials;

struct Avoidee;

#[derive(Bundle)]
struct AvoideeBundle{
    position: Position,
    momentum: Momentum,
    avoidee_marker: Avoidee,

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
            avoidee_marker: Avoidee,
            sprite: SpriteBundle {
                material: (materials).avoidee_material.clone(),
                sprite: Sprite::new(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                ..Default::default()
            }
        }
    );
}