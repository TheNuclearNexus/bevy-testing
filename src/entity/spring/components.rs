use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::defaults;

defaults! {
    #[derive(Debug, Component, Reflect)]
    #[reflect(Component)]
    pub struct SpringConfig {
        pub strength: f32 = 55.0,
        pub unpressed_sprite: i32 = 30,
        pub pressed_sprite: i32 = 15,
    }
}

#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[reflect(Component)]
pub struct SpringTimer(Timer);

impl Default for SpringTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(1.0 / 12.0, TimerMode::Once);
        timer.finish();
        Self(timer)
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct SpringLaunchForce(pub f32);

defaults! {
    #[derive(Debug, Bundle, LdtkEntity)]
    pub struct SpringBundle {
        name: Name = "Spring".into(),
        collider: Collider = Collider::cuboid(4.0, 4.0),

        config: SpringConfig,
        timer: SpringTimer,
        launch_force: SpringLaunchForce,

        #[sprite_sheet]
        sprite_sheet: Sprite,
    }
}
