use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerReloadPosition {
    pub id: LevelIid,
    pub pos: Option<Vec3>,
    pub is_reloading: bool,
}
