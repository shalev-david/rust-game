use bevy::{prelude::{Commands, Entity, GlobalTransform, Query, Transform, With, Mesh}, sprite::Mesh2dHandle};

use crate::{
    combat::components::WeaponEntity, game_entity::GameEntity,
    utils::calculate_weapon_position_and_rotation,
};

use super::components::Equipped;

pub fn attach_weapon_mesh_to_entity(
    mut commanmds: Commands,
    mut entities_equipped: Query<&GlobalTransform, With<Equipped>>,
) {
    entities_equipped
        .iter_mut()
        .map(|entity_global_transform| {});
}

pub fn set_weapon_position_and_rotation(
    entity_query: Query<(&Equipped, &GameEntity)>,
    mut weapon_query: Query<&mut Transform, With<WeaponEntity>>,
) {
    for (weapon_equipped, game_entity) in entity_query.iter() {
        let mut weapon_transform = weapon_query.get_mut(weapon_equipped.weapon).unwrap();
        let (new_transform, angle) = calculate_weapon_position_and_rotation(
            game_entity.entity_position,
            game_entity.aim_position,
            game_entity.width,
            weapon_equipped.weapon_width/2.0,
        );
        *weapon_transform = Transform::from_xyz(new_transform.x, new_transform.y, 0.0);
        weapon_transform.rotate_z(angle);
    }
}
