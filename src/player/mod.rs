use bevy::prelude::Plugin;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(systems::spawn_player)
            .add_system(systems::movement_system);
    }
}
