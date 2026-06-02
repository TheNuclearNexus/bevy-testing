use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{entity::common::components::Groundedness, get_single};

use super::components::Direction;

pub fn update_direction(mut query: Query<(&mut Sprite, &Direction), Changed<Direction>>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction.flip();
    }
}

fn is_grounded(ctx: &RapierContext<'_>, entity: Entity, transform: &Transform) -> bool {
    let center = transform.translation.xy();
    let filter = QueryFilter::default().exclude_collider(entity);
    let max_toi = 4.5;
    let dir = Vec2::new(0.0, -1.0);

    let ray_center = center;
    let ray_left = center + Vec2::new(-3.5, 0.0);
    let ray_right = center + Vec2::new(3.5, 0.0);

    ctx.cast_ray(ray_center, dir, max_toi, true, filter)
        .is_some()
        || ctx.cast_ray(ray_left, dir, max_toi, true, filter).is_some()
        || ctx
            .cast_ray(ray_right, dir, max_toi, true, filter)
            .is_some()
}

pub fn update_groundedness(
    ctx: ReadRapierContext,
    mut query: Query<(Entity, &mut Groundedness, &Transform)>,
) {
    let ctx = get_single!(ctx);

    for (entity, mut groundedness, transform) in query.iter_mut() {
        let new = is_grounded(&ctx, entity, transform);

        let on_ground: bool = *groundedness.as_ref().as_ref();
        
        if on_ground != new {
            groundedness.set(new)
        }
    }
}
