use bevy::prelude::*;
use crate::components::{Position, Momentum};
use crate::constants;

pub fn apply_momentum(query: Query<(&mut Position, &Momentum)>){
    query.for_each_mut(|(mut pos, mom)|{
        pos.0 += mom.0;
    })
}

pub fn loop_space(query: Query<&mut Position>){
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