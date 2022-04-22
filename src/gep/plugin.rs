use bevy::prelude::*;
use crate::game_structs::GameState;
use super::{Position, components::{Collider, ColliderList}, systems};
pub struct GEPPlugin;


impl Plugin for GEPPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ColliderList>()
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
            .label("GEP")
            .with_system(systems::apply_momentum)
            .with_system(systems::resolve_collisions)
        );
    }
}