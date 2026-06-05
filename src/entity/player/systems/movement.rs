use bevy::{ecs::query::QueryData, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    entity::{
        common::components::{Direction, Groundedness},
        player::{PlayerConfig, PlayerCoyoteTimer, PlayerJumpTimer, PlayerState, PlayerWallJumpTimer},
    },
    get_single,
    world::{Group, vines::components::Vines},
};

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
    transform: &'static mut Transform,
    groundedness: &'static Groundedness,
    gravity_scale: &'static mut GravityScale,
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    rapier_context: ReadRapierContext,
    mut query: Query<(Entity, MovementQuery)>,
    vines_query: Query<(&Vines, &Transform), Without<PlayerConfig>>,
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

        let overlapping_vines = is_overlapping_vines(&ctx, player_entity, &vines_query);
        let mut currently_climbing = matches!(*player.state, PlayerState::Climbing { .. });

        if currently_climbing {
            let jump_pressed = keyboard_input.just_pressed(KeyCode::Space);
            if !overlapping_vines || jump_pressed {
                currently_climbing = false;
                *player.gravity_scale = GravityScale(3.5);
                *player.state = PlayerState::Idle;

                if jump_pressed {
                    player.coyote.finish();
                    player.jump.reset();
                    player.impulse.impulse.y = player.config.jump_strength * 1000.0;
                }
            }
        } else if overlapping_vines && player.jump.is_finished() {
            let up_pressed =
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
            let down_pressed =
                keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
            if up_pressed || down_pressed {
                if let Some(vine_x) = get_overlapping_vine_pos(&ctx, player_entity, &vines_query) {
                    currently_climbing = true;
                    *player.gravity_scale = GravityScale(0.0);
                    *player.state = PlayerState::Climbing { x: vine_x };
                }
            }
        }

        if currently_climbing {
            // Get the target X coordinate from the state, falling back to overlapping vine or current x
            let target_x = if let PlayerState::Climbing { x } = *player.state {
                x
            } else if let Some(vine_x) = get_overlapping_vine_pos(&ctx, player_entity, &vines_query) {
                vine_x
            } else {
                player.transform.translation.x
            };

            *player.state = PlayerState::Climbing { x: target_x };
            *player.gravity_scale = GravityScale(0.0);

            // Smoothly move the player to the target X coordinate
            let current_x = player.transform.translation.x;
            let lerp_factor = (15.0 * time.delta_secs()).min(1.0);
            player.transform.translation.x = current_x + (target_x - current_x) * lerp_factor;

            let mut vertical = 0.0_f32;
            if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
                vertical += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
                vertical -= 1.0;
            }

            let mut direction = player.direction.clone();
            if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
                direction = Direction::Left;
            }
            if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
                direction = Direction::Right;
            }

            player.velocity.linear.x = 0.0;
            player.velocity.linear.y = vertical * player.config.speed * 0.375;
            *player.direction = direction;
        } else {
            *player.gravity_scale = GravityScale(3.5);

            let mut horizontal = 0.0_f32;
            let mut state = PlayerState::Idle;
            let mut direction = player.direction.clone();

            let on_ground = *player.groundedness.as_ref();

            // Handle wall jump logic
            let is_wall_jump_locked = !player.wall_jump.is_finished() && !on_ground;

            if is_wall_jump_locked {
                state = PlayerState::Walking;
            } else {
                if keyboard_input.pressed(KeyCode::KeyA)
                    || keyboard_input.pressed(KeyCode::ArrowLeft)
                {
                    horizontal -= 1.0;
                    direction = Direction::Left;
                    state = PlayerState::Walking;
                }
                if keyboard_input.pressed(KeyCode::KeyD)
                    || keyboard_input.pressed(KeyCode::ArrowRight)
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
                player.impulse.impulse.y +=
                    player.config.slide_strength * time.delta_secs() * 1000.0;
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
}

fn get_overlapping_vine_pos(
    ctx: &RapierContext<'_>,
    player_entity: Entity,
    vines_query: &Query<(&Vines, &Transform), Without<PlayerConfig>>,
) -> Option<f32> {
    ctx.intersection_pairs_with(player_entity)
        .find_map(|(entity1, entity2, intersecting)| {
            if intersecting {
                let other = if entity1 == player_entity {
                    entity2
                } else {
                    entity1
                };
                vines_query
                    .get(other)
                    .ok()
                    .map(|(_, transform)| transform.translation.x + 4.0)
            } else {
                None
            }
        })
}

fn is_overlapping_vines(
    ctx: &RapierContext<'_>,
    player_entity: Entity,
    vines_query: &Query<(&Vines, &Transform), Without<PlayerConfig>>,
) -> bool {
    get_overlapping_vine_pos(ctx, player_entity, vines_query).is_some()
}

fn is_against_wall(
    ctx: &RapierContext<'_>,
    player_entity: Entity,
    player: &MovementQueryItem<'_, '_>,
    horizontal: f32,
) -> bool {
    let filter = QueryFilter::default()
        .exclude_collider(player_entity)
        .groups(CollisionGroups::new(Group::all(), Group::GRABBABLE))
        .exclude_sensors();

    let max_toi = 0.1;

    let dir = if horizontal < 0.0 {
        Vec2::new(-1.0, 0.0)
    } else {
        Vec2::new(1.0, 0.0)
    };

    let center = player.transform.translation.xy() + dir * 4.0;

    let mut collider = player.collider.clone();

    collider.set_scale(Vec2::new(0.05, 0.98), 4);

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
