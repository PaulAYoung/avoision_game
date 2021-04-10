use std::{default};
use bevy::prelude::*;
use bevy::math;
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Momentum{
    pub x: f32,
    pub y: f32,
}

impl Momentum{
    /// Sets the velocity while keeping the direction.
    ///
    /// # examples
    ///
    /// ```
    /// use avoision_game::components::Momentum;
    ///
    /// let mut m = Momentum{x:1.0, y: 0.0};
    /// m.set_velocity(10.0);
    /// assert_eq!(m.x, 10.0);
    /// ```
    ///
    /// ```
    /// use avoision_game::components::Momentum;
    ///
    /// let mut m = Momentum{x:3.0, y: 4.0};
    /// m.set_velocity(50.0);
    /// assert_eq!(m.x, 30.0);
    /// assert_eq!(m.y, 40.0);
    /// ```
    pub fn set_velocity(&mut self, velocity: f32){
        let vel = (self.x.powi(2) + self.y.powi(2)).sqrt();
        let change = velocity/vel;
        self.x *= change;
        self.y *= change;
    }
}