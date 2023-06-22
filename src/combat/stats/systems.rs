use bevy::prelude::*;

use crate::player::components::Player;

use super::components::{HitStreak, KillStreak};

pub fn tick_killstreak_timer(mut ks_query: Query<&mut KillStreak, With<Player>>, time: Res<Time>) {
    if let Ok(mut ks) = ks_query.get_single_mut() {
        ks.timer.tick(time.delta());
    }
}

pub fn reset_killstreak_system(mut ks_query: Query<&mut KillStreak>) {
    if let Ok(mut ks) = ks_query.get_single_mut() {
        if ks.timer.finished() {
            ks.streak = 0;
        }
    }
}

pub fn tick_hitstack_timer(mut hs_query: Query<&mut HitStreak, With<Player>>, time: Res<Time>) {
    if let Ok(mut hs) = hs_query.get_single_mut() {
        hs.timer.tick(time.delta());
    }
}

pub fn reset_hitstack_system(mut hs_query: Query<&mut HitStreak, With<Player>>) {
    if let Ok(mut hs) = hs_query.get_single_mut() {
        if hs.timer.finished() {
            hs.streak = 0;
        }
    }
}
