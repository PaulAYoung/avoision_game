use bevy::prelude::*;
#[derive(Default, Clone, Copy, PartialEq, Component)]
pub struct Position( pub Vec2 );

#[derive(Default, Clone, Copy, PartialEq, Component)]
pub struct Momentum( pub Vec2 );

/// Enum for different collision bodies
#[derive(Clone, Copy, PartialEq, Component)]
pub enum Collider {
    Circle{radius: f32},
}

impl Collider {
    pub fn collides(c1: &Collider, c2: &Collider, p1:&Position, p2: &Position)->bool{
        match *c1 {
            Collider::Circle{radius: r} => {
                match *c2{
                    Collider::Circle{radius: r2} => (p1.0 - p2.0).length() <= r+r2,
                }
            },
        }
    }
}

impl Default for Collider{
    fn default() -> Self {
        Collider::Circle { radius: 1.0 }
    }
}

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

pub struct Collision(pub Entity, pub Entity);