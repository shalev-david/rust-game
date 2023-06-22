use bevy::prelude::{default, Camera2dBundle, Commands, Component};

#[derive(Component)]
struct MainCamera;


pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MainCamera {}));
}
