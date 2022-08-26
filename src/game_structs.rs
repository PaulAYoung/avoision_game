use bevy::{prelude::{Deref, DerefMut}};
use bevy::core::Stopwatch;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    InGame,
    Paused
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Score (pub Stopwatch);