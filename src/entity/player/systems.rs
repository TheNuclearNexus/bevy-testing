use bevy::{ecs::query::QueryData, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    entity::{
        common::components::{Direction, Groundedness},
        player::{
            PlayerAnimationTimer, PlayerConfig, PlayerCoyoteTimer, PlayerJumpTimer, PlayerState,
            PlayerWallJumpTimer,
        },
    },
    get_single,
};

#[cfg(feature = "dev")]
use crate::world::PlayerReloadPosition;

pub fn coyote_time(
    mut query: Query<(&Groundedness, &mut PlayerCoyoteTimer), Changed<Groundedness>>,
) {
    for (groundedness, mut coyote) in query.iter_mut() {
        if !*groundedness.as_ref() {
            coyote.reset();
        } else {
            coyote.finish();
        }
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MovementQuery {
    config: &'static PlayerConfig,
    collider: &'static Collider,
    coyote: &'static mut PlayerCoyoteTimer,
    jump: &'static mut PlayerJumpTimer,
    wall_jump: &'static mut PlayerWallJumpTimer,
    state: &'static mut PlayerState,
    velocity: &'static mut Velocity,
    impulse: &'static mut ExternalImpulse,
    force: &'static mut ExternalForce,
    direction: &'static mut Direction,
    transform: &'static Transform,
    groundedness: &'static Groundedness,
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    rapier_context: ReadRapierContext,
    mut query: Query<(Entity, MovementQuery)>,
) {
    let ctx = get_single!(rapier_context);

    for (player_entity, mut player) in &mut query {
        player.coyote.tick(time.delta());
        player.jump.tick(time.delta());
        player.wall_jump.tick(time.delta());

        // Reset the coyote timer if the player is moving up
        if player.velocity.linear.y > 0.0 {
            player.coyote.finish();
        }

        let mut horizontal = 0.0_f32;
        let mut state = PlayerState::Idle;
        let mut direction = player.direction.clone();

        let on_ground = *player.groundedness.as_ref();

        // Handle wall jump logic
        let is_wall_jump_locked = !player.wall_jump.is_finished() && !on_ground;

        if is_wall_jump_locked {
            state = PlayerState::Walking;
        } else {
            if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
                horizontal -= 1.0;
                direction = Direction::Left;
                state = PlayerState::Walking;
            }
            if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
            {
                horizontal += 1.0;
                direction = Direction::Right;
                state = PlayerState::Walking;
            }
        }

        // Wall sliding logic
        let against_wall = is_against_wall(&ctx, player_entity, &player, horizontal);

        let mut is_wall_sliding = false;

        if against_wall
            && !on_ground
            && player.velocity.linear.y < 0.1
            && horizontal.abs() > f32::EPSILON
        {
            is_wall_sliding = true
        }

        if is_wall_sliding && !on_ground {
            state = PlayerState::WallSliding;
            player.impulse.impulse.y += player.config.slide_strength * time.delta_secs() * 1000.0;
        } else if !against_wall && !is_wall_jump_locked {
            player.velocity.linear.x = horizontal * player.config.speed;
        }

        let jump_pressed = keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::KeyW)
            || keyboard_input.just_pressed(KeyCode::ArrowUp);

        if (on_ground || is_wall_sliding || !player.coyote.is_finished())
            && jump_pressed
            && player.jump.is_finished()
        {
            player.coyote.finish();
            player.jump.reset();

            if is_wall_sliding && !on_ground {
                player.wall_jump.reset();

                let jump_dir = if matches!(*player.direction, Direction::Left) {
                    1.0
                } else {
                    -1.0
                };

                player.velocity.linear.x = jump_dir * player.config.speed * 0.75;
                player.velocity.linear.y = 0.0;
            }
            player.impulse.impulse.y = player.config.jump_strength * 1000.0;
        }

        let jump_held = keyboard_input.pressed(KeyCode::Space)
            || keyboard_input.pressed(KeyCode::KeyW)
            || keyboard_input.pressed(KeyCode::ArrowUp);

        if jump_held && !player.jump.is_finished() {
            let length = player.jump.duration().as_secs_f32();
            let multiplier = 1.0 - (player.jump.elapsed_secs() / length);
            player.impulse.impulse.y +=
                player.config.jump_strength * 3000.0 * multiplier * time.delta_secs();
        }

        *player.direction = direction;
        *player.state = state;
    }
}

fn is_against_wall(
    ctx: &RapierContext<'_>,
    player_entity: Entity,
    player: &MovementQueryItem<'_, '_>,
    horizontal: f32,
) -> bool {
    let filter = QueryFilter::default()
        .exclude_collider(player_entity)
        .exclude_sensors();
    let max_toi = 1.5;

    let dir = if horizontal < 0.0 {
        Vec2::new(-1.0, 0.0)
    } else {
        Vec2::new(1.0, 0.0)
    };

    let center = player.transform.translation.xy() + dir * 0.1;

    let mut collider = player.collider.clone();

    collider.set_scale(Vec2::new(1.0, 0.98), 4);

    ctx.cast_shape(
        center,
        player.transform.rotation.to_euler(EulerRot::XYZ).2,
        dir,
        (&collider).into(),
        ShapeCastOptions::with_max_time_of_impact(max_toi),
        filter,
    )
    .is_some()
}

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
        }
    }
}

#[cfg(feature = "dev")]
#[allow(unused)]
pub fn store_position(
    mut reload_pos: ResMut<PlayerReloadPosition>,
    query: Query<&Transform, With<PlayerConfig>>,
) {
    let Ok(player) = query.single() else {
        return;
    };

    reload_pos.pos = Some(player.translation)
}

#[cfg(feature = "dev")]
#[allow(unused)]
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
