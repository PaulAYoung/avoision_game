use bevy::prelude::*;
pub struct Materials{
    pub avoidee_material: Handle<ColorMaterial>,
    pub avoider_material: Handle<ColorMaterial>,
}

pub fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>){
    commands.insert_resource(Materials{
        avoidee_material: materials.add(Color::rgb(0.0, 0.7, 0.7).into()),
        avoider_material: materials.add(Color::rgb(0.7, 0.0, 0.1).into()),
    });
}