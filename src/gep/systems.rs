use bevy::prelude::*;
use super::{Position, Momentum};

pub fn apply_momentum(query: Query<(&mut Position, &Momentum)>){
    query.for_each_mut(|(mut pos, mom)|{
        pos.0 += mom.0;
    })
}