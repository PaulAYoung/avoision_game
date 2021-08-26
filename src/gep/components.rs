use bevy::prelude::*;
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Position( pub Vec2 );

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Momentum( pub Vec2 );

impl Momentum{
    /// Sets the velocity while keeping the direction.
    ///
    /// # examples
    ///
    /// ```
    /// use avoision_game::gep::components::Momentum;
    /// use bevy::prelude::*;
    ///
    /// let mut m = Momentum(Vec2::new(1.0, 0.0));
    /// m.set_velocity(10.0);
    /// assert_eq!(m.0.x, 10.0);
    /// ```
    ///
    /// ```
    /// use avoision_game::gep::components::Momentum;
    /// use bevy::prelude::*;
    ///
    /// let mut m = Momentum(Vec2::new(3.0, 4.0));
    /// m.set_velocity(50.0);
    /// assert_eq!(m.0.x, 30.0);
    /// assert_eq!(m.0.y, 40.0);
    /// ```
    pub fn set_velocity(&mut self, velocity: f32){
        let vel = self.0.length();
        let change = velocity/vel;
        self.0 *= change;
    }
}