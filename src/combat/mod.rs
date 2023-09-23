use bevy::prelude::Plugin;

use self::{
    events::DealDamageEvent,
    stats::systems::{reset_killstreak_system, tick_killstreak_timer},
    systems::damage_system,
    weapons::{firearms::projectile_movement_system, systems::set_weapon_position_and_rotation},
};

pub mod components;
mod events;
pub mod stats;
mod systems;
pub mod weapons;
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DealDamageEvent>()
            .add_system(tick_killstreak_timer)
            .add_system(damage_system)
            .add_system(reset_killstreak_system)
            .add_system(projectile_movement_system)
            .add_system(set_weapon_position_and_rotation);
    }
}
