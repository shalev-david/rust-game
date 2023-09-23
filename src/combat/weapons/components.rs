use bevy::prelude::*;
use super::Weapon;

#[derive(Component)]
pub struct Armed{
    pub weapon: Box<dyn Weapon + Sync + Send>
}

#[derive(Component)]
pub struct Equipped {
    pub weapon: Entity,
    pub weapon_width: f32,
    pub weapon_height: f32,
    pub weapon_pos : Vec2,
    pub weapon_angle : f32
}