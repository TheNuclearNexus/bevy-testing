use bevy::{ecs::query::QueryData, prelude::*};

use crate::entity::{
    common::components::{Direction, Groundedness},
    player::{PlayerAnimationTimer, PlayerConfig, PlayerState},
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimationQuery {
    entity: Entity,
    config: &'static PlayerConfig,
    state: &'static PlayerState,
    groundedness: &'static Groundedness,
    direction: &'static Direction,
    sprite: &'static mut Sprite,
    timer: &'static mut PlayerAnimationTimer,
}

pub fn animations(time: Res<Time>, query: Query<AnimationQuery>) {
    for mut player in query {
        player.timer.tick(time.delta());

        let Some(atlas) = player.sprite.texture_atlas.as_mut() else {
            continue;
        };

        match player.state {
            PlayerState::Idle => atlas.index = player.config.idle_sprite,
            PlayerState::Walking => {
                if !player.timer.just_finished() {
                    continue;
                }

                atlas.index = if atlas.index == player.config.idle_sprite {
                    player.config.moving_sprite
                } else {
                    player.config.idle_sprite
                }
            }
            PlayerState::WallSliding => {
                atlas.index = player.config.idle_sprite;
                player.sprite.flip_x = !player.direction.flip();
            }
            PlayerState::Climbing { .. } => {
                atlas.index = player.config.idle_sprite;
            }
        }
    }
}
