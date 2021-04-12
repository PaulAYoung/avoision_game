use avoider::spawn_avoider;
use bevy::prelude::*;

mod materials;
mod avoider;
mod components;
mod constants;

use components::Position;
use constants::{POSITION_SCALE};


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
        width: 500.0,
        height: 500.0,
        ..Default::default()
    })
    .add_startup_system(materials::setup_materials.system())
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_avoider.system()))
    .add_system(position_scale.system())
    .add_system(avoider::avoider_movement.system())
    .add_plugins(DefaultPlugins).run();
}