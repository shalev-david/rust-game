use bevy::prelude::{Bundle, Component};

use crate::combat::{components::Health, stats::components::CombatStats};

#[derive(Bundle)]
pub struct PlayerBundle {
    _p: Player,
    health: Health,
    combat_stats: CombatStats,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            _p: Player {},
            health: Health { 0: 100.0 },
            combat_stats: CombatStats::default(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Player;
