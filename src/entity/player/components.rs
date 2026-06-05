use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    defaults,
    entity::common::components::{Direction, Groundedness},
};

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

#[derive(Component, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    WallSliding,
    Climbing {
        x: f32,
    },
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
        let mut timer = Timer::from_seconds(1.0 / 9.0, TimerMode::Once);
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

defaults! {
    #[derive(Bundle, LdtkEntity)]
    pub struct PlayerBundle {
        name: Name = "Player".into(),
        rigidbody: RigidBody = RigidBody::Dynamic,
        collider: Collider = Collider::cuboid(4.0, 4.0),
        locked_axes: LockedAxes = LockedAxes::ROTATION_LOCKED,
        gravity_scale: GravityScale = GravityScale(3.5),
        direction: Direction = Direction::Left,
        friction: Friction = Friction::new(0.0),

        config: PlayerConfig,
        state: PlayerState,
        animation_timer: PlayerAnimationTimer,
        coyote_timer: PlayerCoyoteTimer,
        jump_timer: PlayerJumpTimer,
        wall_jump_timer: PlayerWallJumpTimer,
        #[sprite_sheet]
        sprite_sheet: Sprite,
        velocity: Velocity,
        impulse: ExternalImpulse,
        force: ExternalForce,
        groundedness: Groundedness,

        read_mass: ReadMassProperties
    }
}
