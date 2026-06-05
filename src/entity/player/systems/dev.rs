use bevy::prelude::*;

use crate::entity::player::{PlayerConfig, PlayerReloadPosition};

pub fn store_position(
    mut reload_pos: ResMut<PlayerReloadPosition>,
    query: Query<&Transform, With<PlayerConfig>>,
) {
    let Ok(player) = query.single() else {
        return;
    };

    reload_pos.pos = Some(player.translation)
}

pub fn restore_position(
    mut reload_pos: ResMut<PlayerReloadPosition>,
    mut query: Query<&mut Transform, Added<PlayerConfig>>,
) {
    let Ok(mut player) = query.single_mut() else {
        return;
    };

    if let Some(pos) = reload_pos.pos {
        player.translation = pos.clone();
    }
}
