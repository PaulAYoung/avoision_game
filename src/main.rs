use bevy::prelude::*;

mod materials;
mod avoidee;


fn main(){
    App::build()
    .add_resource(WindowDescriptor{
        title: "Avoision".to_string(),
        width: 500.0,
        height: 500.0,
        ..Default::default()
    })
    .add_startup_system(materials::setup_materials.system())
    .add_plugins(DefaultPlugins).run();
}