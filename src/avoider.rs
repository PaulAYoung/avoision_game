use bevy::{prelude::*};
use crate::{gep::{
    Position,
    Momentum
    }, constants::{
        MAX_SPEED,
        AVOIDER_THRUST,
        ITEM_SIZE
    }, materials::{Materials}};
pub struct Avoider;
pub struct AvoiderSpawnEvent{
    pub position: Position,
    pub momentum: Momentum
}

pub fn spawn_avoider_event_reader(
    mut commands: Commands,
    materials: Res<Materials>,
    mut spawn_events: EventReader<AvoiderSpawnEvent>
){
    for e in spawn_events.iter(){
        spawn_avoider(&mut commands, &materials, e.position, e.momentum);
    }
}

pub fn spawn_avoider(
    commands: &mut Commands,
    materials: &Res<Materials>,
    position: Position,
    momentum: Momentum
){
    commands.spawn()
        .insert_bundle(
        SpriteBundle {
            material: (materials).avoider_material.clone(),
            sprite: Sprite::new(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
            ..Default::default()
        }
    )
    .insert(position)
    .insert(momentum)
    .insert(Avoider);
}

pub fn avoider_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut avoider_momentum: Query<&mut Momentum, With<Avoider>>,
    time: Res<Time>
){
    let thrust = AVOIDER_THRUST * time.delta_seconds();
    for mut m in avoider_momentum.iter_mut(){
        let mut impulse = Momentum(Vec2::new(0.0, 0.0));
        if keyboard_input.pressed(KeyCode::Up){
            impulse.0.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down){
            impulse.0.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left){
            impulse.0.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right){
            impulse.0.x += 1.0;
        }
        if impulse.0.length() != 0.0 {
            impulse.set_velocity(thrust);
            m.0 += impulse.0;
            if m.0.length() > MAX_SPEED {
                m.set_velocity(MAX_SPEED);
            }
        }
    }
}