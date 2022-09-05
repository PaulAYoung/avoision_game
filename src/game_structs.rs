use bevy::{prelude::{Deref, DerefMut, Timer}};
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