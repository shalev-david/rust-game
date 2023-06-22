use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::{KinematicCharacterController, Collider, GravityScale};

use super::components::{Player, PlayerBundle};

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands
        .spawn(KinematicCharacterController {
            custom_mass: Some(30.0),
            ..default()
        })
        .insert((
            PlayerBundle::default(),
            TransformBundle::from(Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            )),
            Collider::cuboid(20.0, 40.0),
            GravityScale(0.5),
        ));
}


pub fn movement_system(mut q: Query<&mut KinematicCharacterController, With<Player>>, keyboard_input : Res<Input<KeyCode>>){
    if let Ok(mut transform) = q.get_single_mut(){
        let mut direction = Vec2::ZERO;
        let mut pressed_side = false;
        let mut pressed_up = false;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction.x += -3.0;
            pressed_side = true;
        }
        if  keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction.x += 3.0;
            pressed_side = true;
        }
        if  keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction.y +=  3.0;
            pressed_up = true;
        }
        if  keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction.y +=  -3.0;
            pressed_up = true
        }
        if pressed_side && pressed_up{
            direction *= 0.7   
        }
        if  keyboard_input.pressed(KeyCode::LShift) {
            direction *= 1.2;
        }
        transform.translation = Some(direction);
    }
}