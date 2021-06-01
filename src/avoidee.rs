use bevy::prelude::*;

use crate::components::{Position, Momentum};
use crate::constants::ITEM_SIZE;
use crate::materials::Materials;

struct Avoidee;

pub struct AvoideeSpawnEvent{
    pub position: Position,
    pub momentum: Momentum
}

pub fn spawn_avoidee(
    mut commands: Commands,
    materials: Res<Materials>,
    mut spawn_events: EventReader<AvoideeSpawnEvent>
){
    for e in spawn_events.iter(){
        commands.spawn()
            .insert_bundle(
            SpriteBundle {
                material: materials.avoidee_material.clone(),
                sprite: Sprite::new(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                ..Default::default()
            }
        )
        .insert( e.position)
        .insert(e.momentum)
        .insert(Avoidee);
    }
}