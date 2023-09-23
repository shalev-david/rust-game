use bevy::prelude::*;

const KILL_STREAK_TIMER: f32 = 5.0;
const HIT_STACK_TIMER: f32 = 1.5;

#[derive(Component)]
pub struct CombatStats {
    pub base_damage: f32,
    pub base_attack_speed: f32,
    pub base_life_steal: f32,
    pub shield: f32,
    pub stats: Vec<Stats>,
}

impl Default for CombatStats {
    fn default() -> CombatStats {
        CombatStats {
            base_damage: 1.0,
            base_attack_speed: 1.0,
            base_life_steal: 0.0,
            shield: 0.0,
            stats: vec![
                Stats::KillStreakStat(KillStreak::default()),
                Stats::HitSreakStat(HitStreak::default()),
            ],
        }
    }
}

impl CombatStats {
    pub fn damage_dealt(&mut self, _damage: f32) {
        for stat in self.stats.iter_mut() {
            match stat {
                Stats::HitSreakStat(hs) => hs.damage_dealt(),
                _ => continue,
            }
        }
    }

    pub fn kill_entity(&mut self, _damage: f32) {
        for stat in self.stats.iter_mut() {
            match stat {
                Stats::KillStreakStat(ks) => ks.kill_entity(),
                _ => continue,
            }
        }
    }

    pub fn calculate_damage(&self) -> f32 {
        0.0
    }

    pub fn calculate_attack_speed(&self) -> f32 {
        0.0
    }

    pub fn calculate_life_steal(&self) -> f32 {
        0.0
    }
}

pub enum Stats {
    KillStreakStat(KillStreak),
    HitSreakStat(HitStreak),
}

#[derive(Component, Debug)]
pub struct KillStreak {
    pub streak: u16,
    pub timer: Timer,
}

impl Default for KillStreak {
    fn default() -> KillStreak {
        KillStreak {
            streak: 0,
            timer: Timer::from_seconds(KILL_STREAK_TIMER, TimerMode::Repeating),
        }
    }
}

impl KillStreak {
    fn kill_entity(&mut self) {
        self.streak += 1;
        self.timer.reset()
    }
}

#[derive(Component, Debug)]
pub struct HitStreak {
    pub streak: u16,
    pub timer: Timer,
}

impl Default for HitStreak {
    fn default() -> HitStreak {
        HitStreak {
            streak: 0,
            timer: Timer::from_seconds(HIT_STACK_TIMER, TimerMode::Once),
        }
    }
}

impl HitStreak {
    fn damage_dealt(&mut self) {
        self.streak += 1;
        self.timer.reset()
    }
}
