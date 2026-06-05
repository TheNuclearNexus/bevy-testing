use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

pub mod common;
pub mod player;
pub mod spring;

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
        // Spring
        .register_type::<spring::SpringConfig>()
        .register_type::<spring::SpringTimer>()
        .register_type::<spring::SpringLaunchForce>()
        /* -- Register LDTK entities -- */
        .register_ldtk_entity::<player::PlayerBundle>(player::IDENT)
        .register_ldtk_entity::<spring::SpringBundle>(spring::IDENT)
        /* -- Systems -- */
        .add_systems(
            Update,
            (
                common::systems::update_groundedness,
                player::coyote_time,
                player::movement,
                spring::update_springs,
                common::systems::update_direction,
                player::animations,
            )
                .chain(),
        );

    #[cfg(feature = "dev")]
    app.init_resource::<player::PlayerReloadPosition>()
        .add_systems(
            Update,
            (player::restore_position, player::store_position).chain(),
        );
}
