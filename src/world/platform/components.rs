use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_rapier2d::prelude::*;

use crate::{defaults, world::Group};

defaults! {
    #[derive(Component, Reflect, Debug)]
    #[reflect(Component)]
    pub struct Platform {
        pub allowed_angle: f32 = 45.0
    }
}

defaults! {
    #[derive(Bundle, LdtkIntCell)]
    pub struct PlatformBundle {
        platform: Platform,
        collision_group: CollisionGroups = CollisionGroups::new(
            !Group::GRABBABLE & Group::all(),
            Group::all()
        ),
        collider: Collider = Collider::compound(vec![
            (
                Vec2::new(0.0, 2.0),
                0.0,
                Collider::cuboid(4.0, 2.0)
            )
        ]),
        hooks: ActiveHooks = ActiveHooks::MODIFY_SOLVER_CONTACTS,
    }
}
