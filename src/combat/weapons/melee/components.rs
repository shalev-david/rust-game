use bevy::prelude::*;

use crate::combat::weapons::Weapon;

pub trait Melee {}
impl Weapon for dyn Melee {
    fn attack(
        &self,
        commands: Commands,
        dealer: Entity,
        aim_position: Vec2,
        entity_position: Vec2,
    ) {
        todo!()
    }
    fn equip(
        &self,
        commands: Commands,
        entity: Entity,
        entity_pos: Vec2,
        entity_half_width: f32,
        entity_aim_position: Vec2,
        asset_server: Res<AssetServer>,
    ) {
        todo!()
    }
    fn unequip(&self, commands: Commands) {
        todo!()
    }
    fn spawn(&self, commands: Commands, spawn_pos: Transform) {
        todo!()
    }
}

#[derive(Component)]
pub struct Sword {}
