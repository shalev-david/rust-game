use bevy::prelude::*;

#[derive(Component)]
pub struct Weapon{
    pub damage: f32,
    pub attack_speed: f32,
    pub weapon_type: WeaponType
}

pub enum WeaponType{
    Gun,
    Melee
}


#[derive(Component)]
pub struct Projectile{
    damage: f32,
    speed: f32,
    dealer: Entity
}


