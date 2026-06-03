use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::entity::player::PlayerBundle;

pub mod player;
pub mod common;

pub fn plugin(app: &mut App) {
    app
        /* -- Register Types -- */
        // Common
        .register_type::<common::components::Direction>()
        .register_type::<common::components::Groundedness>()
        // Player
        .register_type::<player::PlayerAnimationTimer>()
        .register_type::<player::PlayerConfig>()
        .register_type::<player::PlayerCoyoteTimer>()
        .register_type::<player::PlayerJumpTimer>()
        .register_type::<player::PlayerWallJumpTimer>()
        .register_type::<player::PlayerState>()
        /* -- Register LDTK entities -- */
        .register_ldtk_entity::<PlayerBundle>("Player")
        /* -- Systems -- */
        .add_systems(Update, (
            common::systems::update_groundedness,
            player::coyote_time,
            player::movement, 
            common::systems::update_direction,
            player::animations,
        ).chain());
}
