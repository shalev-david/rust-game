use bevy::{
    prelude::{Assets, Commands, Entity, Mesh, ResMut, Transform, Vec2, AssetServer, Res},
    sprite::ColorMaterial,
};

pub mod components;
pub mod firearms;
pub mod systems;

mod melee;

pub trait Weapon {
    fn attack(&self, commands: Commands, dealer: Entity, aim_position: Vec2, entity_position: Vec2);
    fn equip(
        &self,
        commands: Commands,
        entity: Entity,
        entity_pos: Vec2,
        entity_half_width: f32,
        entity_aim_position: Vec2,
        asset_server: Res<AssetServer>,
    );
    fn unequip(&self, commands: Commands);
    fn spawn(&self, commands: Commands, spawn_pos: Transform);
}

pub enum WeaponType {
    Melee,
    Firearm,
    Rouge,
    Spell,
}
