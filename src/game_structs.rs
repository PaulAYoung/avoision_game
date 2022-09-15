use bevy::{prelude::{Deref, DerefMut, Timer, Component}};
use bevy::core::Stopwatch;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    InGame,
    Paused,
    GameOver
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Score (pub Stopwatch);
#[derive(Default, Component)]
pub struct GameEntity;