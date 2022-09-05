use std::thread::spawn;

use bevy::core::Stopwatch;
use bevy::prelude::*;
use rand::prelude::random;

use crate::gep::components::Collision;
use crate::gep::{Position, Momentum};
use crate::constants::{self, ARENA_HEIGHT, ARENA_WIDTH};
use crate::avoider::{AvoiderSpawnEvent, Avoider};
use crate::avoidee::{AvoideeSpawnEvent};

use crate::game_structs::GameState;

pub fn setup_game(
    mut avoider_spawn: EventWriter<AvoiderSpawnEvent>,
    mut avoidee_spawn: EventWriter<AvoideeSpawnEvent>
){
    avoider_spawn.send(AvoiderSpawnEvent{
        position: Position(Vec2::new(0.0, 0.0)),
        momentum: Momentum(Vec2::new(0.0, 0.0)),
    });

    avoidee_spawn.send(AvoideeSpawnEvent{
        position: Position(Vec2::new(
            random::<f32>()*ARENA_WIDTH as f32,
            random::<f32>()*ARENA_HEIGHT as f32
        )),
        momentum: Momentum(
            Vec2::new(
                random::<f32>()*random::<f32>()*random::<f32>(),
                random::<f32>()*random::<f32>()*random::<f32>())
        )
    });
}

pub fn loop_space(mut query: Query<&mut Position>){
    query.for_each_mut(|mut pos|{
        if pos.0.y > (constants::ARENA_HEIGHT as f32)/2.0{
            pos.0.y = (constants::ARENA_HEIGHT as f32)/-2.0;
        } else if pos.0.y < (constants::ARENA_HEIGHT as f32)/-2.0 {
            pos.0.y = (constants::ARENA_HEIGHT as f32)/2.0;
        }

        if pos.0.x > (constants::ARENA_WIDTH as f32)/2.0{
            pos.0.x = (constants::ARENA_WIDTH as f32)/-2.0;
        } else if pos.0.x < (constants::ARENA_WIDTH as f32)/-2.0 {
            pos.0.x = (constants::ARENA_WIDTH as f32)/2.0;
        }
    })
}

pub fn pause_unpause(
    mut game_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::P){
        match game_state.current(){
            GameState::InGame => {
                game_state.push(GameState::Paused).unwrap();
            }
            GameState::Menu => {
                game_state.set(GameState::Paused).unwrap();
            }
            GameState::Paused => {
                game_state.pop().unwrap();
            }
            _ => {
            }
        }
    }
}

pub fn detect_player_collision(
    query: Query<Entity, With<Avoider>>,
    mut collisions: EventReader<Collision>,
    mut game_state: ResMut<State<GameState>>,
){
    if !query.is_empty(){
        let player_entity = query.single();
        for c in collisions.iter(){
            if c.0 == player_entity || c.1 == player_entity{
                game_state.push(GameState::GameOver).unwrap()
            }
        }
    }
}

pub fn avoidee_spawner(
    time:Res<Time>,
    mut spawn_timer: Local<Stopwatch>,
    mut avoidee_spawn: EventWriter<AvoideeSpawnEvent>
){
    spawn_timer.tick(time.delta());
    if spawn_timer.elapsed_secs() > 3.0 {
        println!("Spawn!");
        spawn_timer.reset();
        avoidee_spawn.send(AvoideeSpawnEvent{
            position: Position(Vec2::new(
                random::<f32>()*ARENA_WIDTH as f32,
                random::<f32>()*ARENA_HEIGHT as f32
            )),
            momentum: Momentum(
                Vec2::new(
                    random::<f32>()*random::<f32>()*random::<f32>()*2.0,
                    random::<f32>()*random::<f32>()*random::<f32>()*2.0)
            )
        });

    }
}