use bevy::prelude::{AssetServer, Commands, Component, Entity, Handle, Res, Transform, Vec2};

use crate::{combat::weapons::Weapon, utils};

use super::{Firearm, Projectile};

#[derive(Component, Debug)]
pub struct Pistol {
    pub damage: f32,
    pub attack_speed: f32,
    pub reload_time: f32,
    pub bullet_speed: f32,
    pub mag_current: u32,
    pub mag_cap: u32,
    pub range: f32,
    pub texture_path: String,
    pub width: f32,
    pub height: f32
}

impl Pistol {}

const PISTOL_WIDTH: f32 = 20.0;
const PISTOL_HEIGHT: f32 = 10.0;

unsafe impl Send for Pistol {}

unsafe impl Sync for Pistol {}

impl Weapon for Pistol {
    fn attack(
        &self,
        mut commands: Commands,
        dealer: Entity,
        aim_position: Vec2,
        entity_position: Vec2,
    ) {
        let projectile_bundle = Projectile::new(
            self.damage,
            self.bullet_speed,
            entity_position,
            aim_position,
            self.range,
            super::ProjectileEffect::Regular,
            dealer,
        );
        println!("Spawned Projectile! |  {:?}", &projectile_bundle.projectile);
        commands.spawn(projectile_bundle);
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
        utils::equip_weapon(
            commands,
            entity,
            entity_pos,
            entity_half_width,
            entity_aim_position,
            self.width/2.0,
            self.height,
            asset_server.load(self.texture_path.clone()),
        );
        println!("Equipped a pistol! {:?}", self)
    }

    fn unequip(&self, commands: Commands) {
        todo!()
    }

    fn spawn(&self, commands: Commands, spawn_pos: Transform) {
        todo!()
    }
}

#[deny(implied_bounds_entailment)]
impl Firearm for Pistol {
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

impl Pistol {
    pub fn generate_test() -> Self {
        Pistol {
            damage: 1.0,
            attack_speed: 2.0,
            reload_time: 1.0,
            bullet_speed: 6000.0 / 60.0,
            mag_current: 10,
            mag_cap: 10,
            range: 25500.0,
            texture_path: "images/tmp_shutgon.png".to_owned(),
            width: 100.0,
            height: 25.0
        }
    }
}
