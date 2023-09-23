use bevy::{
    prelude::{
        Camera, Camera2dBundle, Commands, Component, GlobalTransform, Plugin, Query,
        Resource, Vec2, With,
    },
    window::Window,
};

use crate::{game_entity::GameEntity, player::components::Player};

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

impl Default for CursorPosition {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

fn cursor_position_system(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_query: Query<&mut GameEntity, With<Player>>
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        player_query.single_mut().aim_position = Vec2::new(cursor_position.x, cursor_position.y);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        MainCamera {},
    ));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CursorPosition>()
            .add_startup_system(spawn_camera)
            .add_system(cursor_position_system);
    }
}
