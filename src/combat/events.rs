use bevy::prelude::*;

pub struct DealDamageEvent{
    pub damage: f32,
    pub damage_dealer: Entity,
    pub damage_receiver: Entity
}