use avoidee::spawn_avoidee;
use bevy::prelude::*;

mod materials;
mod avoidee;
mod components;
mod constants;

use components::Position;
use constants::{POSITION_SCALE};


fn position_scale(mut q: Query<(&Position, &mut Transform)>){
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            pos.x as f32 * POSITION_SCALE,
            pos.y as f32 * POSITION_SCALE,
            0.0,
        );
    }
}

fn setup(commands: &mut Commands){
    commands.spawn(Camera2dBundle::default());
}
fn main(){
    App::build()
    .add_resource(WindowDescriptor{
        title: "Avoision".to_string(),
        width: 500.0,
        height: 500.0,
        ..Default::default()
    })
    .add_startup_system(materials::setup_materials.system())
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_avoidee.system()))
    .add_system(position_scale.system())
    .add_system(avoidee::avoidee_movement.system())
    .add_plugins(DefaultPlugins).run();
}