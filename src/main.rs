use avoidee::{AvoideeSpawnEvent, spawn_avoidee_event_reader};
use avoider::{AvoiderSpawnEvent, spawn_avoider_event_reader};
use bevy::prelude::*;

mod materials;
mod avoider;
mod avoidee;
mod constants;
mod systems;

mod gep;

use gep::{Position};
use constants::{POSITION_SCALE, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    InGame,
    Paused
}

fn position_scale(mut q: Query<(&Position, &mut Transform)>){
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = (pos.0*POSITION_SCALE).extend(0.0);
    }
}

fn setup(mut commands: Commands){
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
fn main(){
    App::build()
    .insert_resource(WindowDescriptor{
        title: "Avoision".to_string(),
        width: ARENA_WIDTH as f32 *POSITION_SCALE,
        height: ARENA_HEIGHT as f32 *POSITION_SCALE,
        ..Default::default()
    })
    .add_state(GameState::InGame)
    .add_event::<AvoiderSpawnEvent>()
    .add_event::<AvoideeSpawnEvent>()
    .add_startup_system(materials::setup_materials.system())
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup",
        SystemStage::parallel()
        .with_system(systems::setup_game.system())
    )
    .add_system_set(
        SystemSet::on_update(GameState::InGame)
        .with_system(spawn_avoider_event_reader.system())
        .with_system(spawn_avoidee_event_reader.system())
        .with_system(position_scale.system())
        .with_system(avoider::avoider_movement.system())
        .with_system(systems::apply_momentum.system())
        .with_system(systems::loop_space.system())
    )
    .add_system_set(
        SystemSet::on_update(GameState::Paused)
    )
    .add_system(systems::pause_unpause.system())
    .add_plugins(DefaultPlugins)
    .run();
}