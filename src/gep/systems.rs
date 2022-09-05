use bevy::prelude::*;
use super::{Position, Momentum, Collider, components::Collision};
use crate::{avoider::Avoider};

pub fn apply_momentum(
    mut query: Query<(&mut Position, &Momentum)>
    ){
    query.for_each_mut(|(mut pos, mom)|{
        pos.0 += mom.0;
    })
}

pub fn detect_collisions(
    query: Query<(Entity, &Position, &Collider, &Momentum)>,
    mut collisions: EventWriter<Collision>
){
    let mut iter = query.iter_combinations();
    while let Some([(e1, p1, c1, m1), (e2, p2, c2, m2)])
    = iter.fetch_next(){
        if Collider::collides(c1, c2, p1, p2){
            collisions.send(Collision((e1), (e2)));
        }
    }
}

pub fn resolve_collisions(
    mut query: Query<(&Position, &mut Momentum)>,
    mut collisions: EventReader<Collision>
){
    for c in collisions.iter(){
        let [(p1, mut m1), (p2, mut m2)] = query.many_mut([c.0, c.1]);
        let c = Momentum((p1.0-p2.0).normalize());
        let l1 = m1.0.length();
        let l2 = m2.0.length();
        m1.0 += c.0 * l2 * 0.5;
        m2.0 -= c.0 * l1 * 0.5;
    }
}
