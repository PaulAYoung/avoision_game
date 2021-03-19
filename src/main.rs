use bevy::prelude::*;



fn main(){
    App::build()
    .add_resource(WindowDescriptor{
        title: "Avoision".to_string(),
        width: 500.0,
        height: 500.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins).run();
}