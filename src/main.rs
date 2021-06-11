use avoidee::{AvoideeSpawnEvent, spawn_avoidee};
use avoider::{AvoiderSpawnEvent, spawn_avoider_event_reader};
use bevy::prelude::*;

mod materials;
mod avoider;
mod avoidee;
mod components;
mod constants;
mod systems;

use components::Position;
use constants::{POSITION_SCALE, ARENA_HEIGHT, ARENA_WIDTH};


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
    .add_event::<AvoiderSpawnEvent>()
    .add_event::<AvoideeSpawnEvent>()
    .add_startup_system(materials::setup_materials.system())
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup",
        SystemStage::parallel()
        .with_system(systems::setup_game.system())
    )
    .add_system(spawn_avoider_event_reader.system())
    .add_system(spawn_avoidee.system())
    .add_system(position_scale.system())
    .add_system(avoider::avoider_movement.system())
    .add_system(systems::apply_momentum.system())
    .add_system(systems::loop_space.system())
    .add_plugins(DefaultPlugins).run();
}