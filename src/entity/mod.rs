use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::entity::player::PlayerBundle;

pub mod player;
pub mod common;

pub fn plugin(app: &mut App) {
    app
        .register_type::<player::PlayerConfig>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Update, (
            common::systems::update_groundedness,
            player::coyote_time,
            player::movement, 
            common::systems::update_direction,
            player::animations,
        ).chain());
}
