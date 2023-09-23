use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody};

use crate::{
    combat::weapons::{
        components::{Armed, Equipped},
        firearms::{calculate_projectile_startoff, pistol::Pistol},
    },
    game_entity::GameEntity,
};

use super::components::{Player, PlayerBundle, PLAYER_HEIGHT, PLAYER_WIDTH};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(RigidBody::KinematicPositionBased).insert((
        KinematicCharacterController::default(),
        PlayerBundle::default(),
        TransformBundle::from(Transform::from_translation(Vec3::ZERO)),
        Collider::cuboid(PLAYER_WIDTH, PLAYER_HEIGHT),
    ));
}

pub fn movement_system(
    mut player_query: Query<(&mut KinematicCharacterController, Option<&Equipped>), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut character_control, weapon_entity) = player_query.single_mut();
    let mut direction = Vec2::ZERO;
    let mut pressed_side = false;
    let mut pressed_up = false;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction.x += -3.0;
        pressed_side = true;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction.x += 3.0;
        pressed_side = true;
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction.y += 3.0;
        pressed_up = true;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction.y += -3.0;
        pressed_up = true
    }
    if pressed_side && pressed_up {
        direction *= 0.7
    }
    if keyboard_input.pressed(KeyCode::LShift) {
        direction *= 1.2;
    }
    character_control.translation = Some(direction);
}

pub fn player_attack_system(
    mouse_clicked: Res<Input<MouseButton>>,
    mut player_query: Query<(&GameEntity, &mut Armed, Entity), (With<Player>, With<Equipped>)>,
    commands: Commands,
) {
    if mouse_clicked.just_pressed(MouseButton::Left) {
        let (game_entity, weapon, entity) = player_query.single_mut();

        weapon.weapon.attack(
            commands,
            entity,
            game_entity.aim_position,
            calculate_projectile_startoff(
                game_entity.entity_position,
                game_entity.aim_position,
                game_entity.width,
                game_entity.height,
            ),
        );
    }
}

pub fn update_player_position_system(
    mut player_query: Query<(&mut GameEntity, &GlobalTransform), With<Player>>,
) {
    let (mut game_entity, transform) = player_query.single_mut();
    //println!("Global Transform: {:?}", transform.translation());
    game_entity.entity_position = Vec2::new(transform.translation().x, transform.translation().y);
}

pub fn player_equip_system(
    commands: Commands,
    player_query: Query<(&GameEntity, Entity, &Armed), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    let (game_entity, entity, armed) = player_query.single();
    if keyboard_input.just_pressed(KeyCode::E) {
        armed.weapon.equip(
            commands,
            entity,
            game_entity.entity_position,
            PLAYER_WIDTH,
            game_entity.aim_position,
            asset_server,
        );
    }
}
