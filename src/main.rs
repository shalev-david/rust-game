use bevy::{ecs::system::Commands, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    prelude::{Collider, NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use rust_game::{camera::*, player::PlayerPlugin, combat::CombatPlugin};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PlayerPlugin{})
        .add_plugin(CombatPlugin{})
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_floor)
        // .add_system(combat_system)
        .run();
}

pub fn spawn_floor(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let w = window.get_single().unwrap();

    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            w.width() / 8.0,
            w.height() / 5.0,
            0.0,
        )));
}
