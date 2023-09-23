use bevy::prelude::*;

use crate::combat::weapons::Weapon;

use super::Firearm;

#[derive(Component)]
pub struct Shotgun {
    pub damage: f32,
    pub attack_speed: f32,
    pub reload_time: f32,
    pub bullet_speed: f32,
    pub mag_current: u32,
    pub mag_cap: u32,
    pub number_of_projectiles: u8,
    pub range: f32,
}

unsafe impl Send for Shotgun {}

unsafe impl Sync for Shotgun {}

impl Weapon for Shotgun {
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

impl Firearm for Shotgun {
    fn get_mag_cap(&self) -> u32 {
        self.mag_cap
    }

    fn get_mag_curr(&self) -> u32 {
        self.mag_current
    }

    fn get_reload_time(&self) -> f32 {
        self.reload_time
    }

    fn get_attack_speed(&self) -> f32 {
        self.attack_speed
    }

    fn get_bullet_speed(&self) -> f32 {
        self.bullet_speed
    }

    fn get_bullet_damage(&self) -> f32 {
        self.damage
    }

    fn decrease_mag(&mut self) {
        self.mag_current -= 1;
    }

    fn get_range(&self) -> f32 {
        self.range
    }

    fn get_collider(&self) -> &bevy_rapier2d::prelude::Collider {
        todo!()
    }
}
