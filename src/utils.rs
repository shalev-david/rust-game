use crate::combat::{components::WeaponEntity, weapons::components::Equipped};
use bevy::{
    prelude::{
        default, shape, AssetServer, Assets, Color, Commands, Entity, Mesh, Res, ResMut, Transform,
        Vec2, Image, Handle,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, SpriteBundle, Sprite},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::Collider;

pub fn equip_weapon(
    mut commands: Commands,
    entity: Entity,
    entity_pos: Vec2,
    entity_half_width: f32,
    entity_aim_position: Vec2,
    weapon_half_width: f32,
    weapon_height: f32,
    texture: Handle<Image>
) {
    let (position, angle) = calculate_weapon_position_and_rotation(
        entity_pos,
        entity_aim_position,
        entity_half_width,
        weapon_half_width,
    );
    let mut transform = Transform::from_xyz(position.x, position.y, 0.0);
    transform.rotate_z(angle);
    let weapon_entity = commands
        .spawn((
            SpriteBundle{
                texture,
                transform,
                ..default()
            },
            WeaponEntity,
        ))
        .id();

    commands.get_entity(entity).unwrap().insert(Equipped {
        weapon: weapon_entity,
        weapon_width: weapon_half_width*2.0,
        weapon_height,
        weapon_pos: position,
        weapon_angle: angle,
    });
}

pub fn calculate_weapon_position_and_rotation(
    entity_pos: Vec2,
    entity_aim_position: Vec2,
    entity_half_width: f32,
    weapon_half_width: f32,
) -> (Vec2, f32) {
    let translation: Vec2;
    if entity_aim_position.x > entity_pos.x {
        translation = Vec2::new(
            entity_pos.x + entity_half_width + weapon_half_width,
            entity_pos.y,
        );
    } else {
        translation = Vec2::new(
            entity_pos.x - entity_half_width - weapon_half_width,
            entity_pos.y,
        );
    }

    let mut angle = (Vec2 { x: entity_aim_position.x, y: translation.y } - translation).angle_between(entity_aim_position - translation);
    if angle >= 1.15{
        angle = 1.15

    }else if angle <= -1.15{
        angle = -1.15
    }
    (translation, angle)
}
