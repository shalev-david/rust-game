use super::{
    components::Health,
    events::DealDamageEvent,
    stats::{
        components::CombatStats,
    },
};
use crate::player::components::Player;
use bevy::prelude::*;

pub fn damage_system(
    mut damage_dealt_event: EventReader<DealDamageEvent>,
    mut health_query: Query<&mut Health>,
    mut commands: Commands,
    mut combat_stats_query: Query<&mut CombatStats, With<Player>>,
) {
    for &DealDamageEvent {
        damage,
        damage_dealer,
        damage_receiver,
    } in damage_dealt_event.iter()
    {
        let mut health = health_query.get_mut(damage_receiver).unwrap();
        if let Ok(mut combat_stats) = combat_stats_query.get_mut(damage_dealer) {
            combat_stats.damage_dealt(damage)
        }
        if health.0 < damage {
            commands.entity(damage_receiver).despawn();
            if let Ok(mut combat_stats) = combat_stats_query.get_mut(damage_dealer){
                combat_stats.kill_entity(damage)
            }
            continue;
        }
        health.0 -= damage;
    }
}
