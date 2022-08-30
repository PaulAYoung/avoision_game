use bevy::prelude::*;
use crate::game_structs::{GameState};
use super::{Position, components::{Collider, Collision}, systems::{self, detect_collisions}};
pub struct GEPPlugin;


impl Plugin for GEPPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_event::<Collision>()
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
            .label("GEP")
            .with_system(systems::apply_momentum)
            .with_system(systems::detect_collisions)
            .with_system(systems::resolve_collisions.after(detect_collisions))
        );
    }
}