use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{entity::common::components::Groundedness, get_single};

use super::components::Direction;

pub fn update_direction(mut query: Query<(&mut Sprite, &Direction), Changed<Direction>>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction.flip();
    }
}

fn is_grounded(
    ctx: &RapierContext<'_>,
    entity: Entity,
    collider: &Collider,
    transform: &Transform,
) -> bool {
    let center = transform.translation.xy();
    let filter = QueryFilter::default().exclude_collider(entity);
    let max_toi = 1.0;
    let dir = Vec2::new(0.0, -1.0);

    let mut collider = collider.clone();
    collider.set_scale(Vec2::new(0.98, 1.0), 4);
    ctx.cast_shape(
        center,
        transform.rotation.to_euler(EulerRot::XYZ).2,
        dir,
        (&collider).into(),
        ShapeCastOptions::with_max_time_of_impact(max_toi),
        filter,
    )
    .is_some()
}

pub fn update_groundedness(
    ctx: ReadRapierContext,
    mut query: Query<(Entity, &mut Groundedness, &Collider, &Transform)>,
) {
    let ctx = get_single!(ctx);

    for (entity, mut groundedness, collider, transform) in query.iter_mut() {
        let new = is_grounded(&ctx, entity, collider, transform);

        let on_ground: bool = *groundedness.as_ref().as_ref();

        if on_ground != new {
            groundedness.set(new)
        }
    }
}
