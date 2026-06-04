use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::defaults;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Foliage;

#[derive(Clone, Bundle)]
pub struct FoliageBundle(Foliage);

impl LdtkIntCell for FoliageBundle {
    fn bundle_int_cell(_cell: IntGridCell, _layer: &LayerInstance) -> Self {
        Self(Foliage)
    }
}

defaults! {
    #[derive(Clone, Eq, PartialEq, Debug, Component)]
    pub struct FoliageSensor {
        pub timer: Timer = Timer::from_seconds(0.1, TimerMode::Once),

        pub intersecting: bool,
    }
}

impl FoliageSensor {
    pub fn finished() -> Self {
        let mut sensor = Self::default();
        sensor.timer.finish();
        sensor
    }
}

defaults! {
    #[derive(Clone, Bundle)]
    pub struct FoliageSensorBundle {
        pub name: Name = "Foliage Sensor".into(),
        pub foliage: FoliageSensor = FoliageSensor::finished(),
        pub collider: Collider = Collider::cuboid(3.0, 3.0),
        pub sensor: Sensor,
        pub active_events: ActiveEvents = ActiveEvents::empty(),
    }
}
