use bevy::prelude::*;
use super::{Position, Momentum, Collider};

pub fn apply_momentum(
    mut query: Query<(&mut Position, &Momentum)>
    ){
    query.for_each_mut(|(mut pos, mom)|{
        pos.0 += mom.0;
    })
}

pub fn resolve_collisions(
    mut query: Query<(&Position, &Collider, &mut Momentum)>
){
    let mut iter = query.iter_combinations_mut();
    while let Some([(p1, c1, mut m1), (p2, c2, mut m2)])
    = iter.fetch_next(){
        if Collider::collides(c1, c2, p1, p2){
            let c = Momentum((p1.0-p2.0).normalize());
            let l1 = m1.0.length();
            let l2 = m2.0.length();
            m1.0 += c.0 * l2 * 0.5;
            m2.0 -= c.0 * l1 * 0.5;
        }
    }
}