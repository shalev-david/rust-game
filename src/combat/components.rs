use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Cooldown {
    pub timer: Timer,
    started_once: bool,
}

impl Cooldown {
    pub fn new(duration_in_seconds: f32, timer_mode: TimerMode) -> Cooldown {
        Cooldown {
            timer: Timer::from_seconds(duration_in_seconds, timer_mode),
            started_once: false,
        }
    }

    pub fn available(&mut self) -> bool {
        if !self.started_once {
            self.started_once = true;
            return true;
        }
        if self.timer.finished() {
            self.timer.reset();
            return true;
        }
        false
    }

    pub fn reset(&mut self) {
        self.timer.reset()
    }
    pub fn pause(&mut self) {
        self.timer.pause();
    }
    pub fn unpase(&mut self) {
        self.timer.unpause();
    }
}

#[derive(Component)]
pub struct WeaponEntity;