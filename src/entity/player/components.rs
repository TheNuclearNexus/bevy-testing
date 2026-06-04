use bevy::prelude::*;

use crate::defaults;

defaults! {
    #[derive(Debug, Component, Reflect)]
    #[reflect(Component)]
    pub struct PlayerConfig {
        pub speed: f32 = 100.0,
        pub jump_strength: f32 = 5.0,
        pub slide_strength: f32 = 10.0,

        pub idle_sprite: usize = 91,
        pub moving_sprite: usize = 92,
    }
}

#[derive(Component, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    WallSliding,
}

#[derive(Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimationTimer(Timer);

impl Default for PlayerAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0 / 6.0, TimerMode::Repeating))
    }
}

#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerCoyoteTimer(Timer);

impl Default for PlayerCoyoteTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(1.0 / 6.0, TimerMode::Once);
        timer.finish();
        Self(timer)
    }
}

#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerJumpTimer(Timer);

impl Default for PlayerJumpTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(1.0 / 3.0, TimerMode::Once);
        timer.finish();
        Self(timer)
    }
}

#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerWallJumpTimer(Timer);

impl Default for PlayerWallJumpTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(1.0 / 7.0, TimerMode::Once);
        timer.finish();
        Self(timer)
    }
}