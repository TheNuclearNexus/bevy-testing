use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::defaults;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Vines;

defaults! {
    #[derive(Bundle, LdtkIntCell)]
    pub struct VinesBundle {
        vines: Vines,
        collider: Collider = Collider::cuboid(4.0, 4.0),
        sensor: Sensor = Sensor,
        rigidbody: RigidBody = RigidBody::Fixed,
    }
}
