use bevy::prelude::{Bundle, Component, Resource, Vec2};

use crate::combat::weapons::components::Armed;
use crate::combat::weapons::firearms::pistol::Pistol;
use crate::combat::{components::Health, stats::components::CombatStats};
use crate::game_entity::GameEntity;
#[derive(Bundle)]
pub struct PlayerBundle {
    pub _p: Player,
    pub health: Health,
    pub combat_stats: CombatStats,
    pub armed: Armed,
    pub game_entity: GameEntity,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            _p: Player,
            health: Health { 0: 100.0 },
            combat_stats: CombatStats::default(),
            armed: Armed {
                weapon: Box::new(Pistol::generate_test()),
            },
            game_entity: GameEntity {
                width: PLAYER_WIDTH,
                height: PLAYER_HEIGHT,
                entity_position: Vec2::ZERO,
                aim_position: Vec2::ZERO,
            },
        }
    }
}

#[derive(Component, Debug)]
pub struct Player;

pub const PLAYER_WIDTH: f32 = 20.0;
pub const PLAYER_HEIGHT: f32 = 40.0;
pub const SPAWN_X: f32 = 0.0;
pub const SPAWN_Y: f32 = 0.0;
