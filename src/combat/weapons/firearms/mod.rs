use bevy::{
    prelude::{Bundle, Commands, Component, Entity, Query, Transform, Vec2},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use super::Weapon;

pub mod pistol;
pub mod shotgun;

pub trait Firearm: Weapon + Sync + Send {
    fn get_mag_cap(&self) -> u32;
    fn get_mag_curr(&self) -> u32;
    fn get_reload_time(&self) -> f32;
    fn get_attack_speed(&self) -> f32;
    fn get_bullet_speed(&self) -> f32;
    fn get_bullet_damage(&self) -> f32;
    fn get_range(&self) -> f32;
    fn get_collider(&self) -> &Collider;
    fn decrease_mag(&mut self);
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    transform: TransformBundle,
    rigid_body: RigidBody,
    velocity: Velocity,
    collider: Collider,
}

#[derive(Component, Debug)]
pub struct Projectile {
    damage: f32,
    effect: ProjectileEffect,
    dealer: Entity,
    range: f32,
    traveled: f32,
    direction: Vec2,
    speed: f32,
}

impl Projectile {
    fn new(
        damage: f32,
        speed: f32,
        start_position: Vec2,
        aim_position: Vec2,
        range: f32,
        effect: ProjectileEffect,
        dealer: Entity,
    ) -> ProjectileBundle {
        let direction = (aim_position - start_position).normalize_or_zero() * speed;
        let traveled = 0.0;
        ProjectileBundle {
            projectile: Projectile {
                damage,
                effect,
                dealer,
                range,
                traveled,
                direction,
                speed,
            },
            transform: TransformBundle::from(Transform::from_xyz(
                start_position.x,
                start_position.y,
                0.0,
            )),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: Vec2::new(direction.x, direction.y),
                angvel: 0.0,
            },
            collider: Collider::ball(1.0),
        }
    }
}

pub fn projectile_movement_system(
    mut commands: Commands,
    mut projectile_query: Query<(&mut Projectile, &mut Velocity, Entity)>,
) {
    for (mut projectile, mut velocity, entity) in projectile_query.iter_mut() {
        velocity.linvel = projectile.direction;
        projectile.traveled += projectile.speed;
        if projectile.traveled >= projectile.range {
            commands.entity(entity).despawn();
        }
    }
}

pub fn calculate_projectile_startoff(
    entity_pos: Vec2,
    aim_position: Vec2,
    entity_x_size: f32,
    entity_y_size:f32
) -> Vec2 {
    (aim_position - entity_pos).normalize_or_zero() + entity_pos
    
}


#[derive(Debug)]
pub enum ProjectileEffect {
    Regular,
}
