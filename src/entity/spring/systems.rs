use std::cell::RefCell;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entity::spring::SpringLaunchForce;
use crate::entity::spring::components::{SpringConfig, SpringTimer};
use crate::get_single;

pub fn update_springs(
    time: Res<Time>,
    rapier_context: ReadRapierContext,
    mut spring_query: Query<(
        Entity,
        &SpringConfig,
        &mut SpringTimer,
        &mut Collider,
        &mut Sprite,
        &mut SpringLaunchForce,
        &Transform,
    )>,
    mass_query: Query<&ReadMassProperties, Without<SpringConfig>>,
    mut impulse_query: Query<&mut ExternalImpulse, Without<SpringConfig>>,
    mut velocity_query: Query<&mut Velocity, Without<SpringConfig>>,
    mut transform_query: Query<&mut Transform, Without<SpringConfig>>,
) {
    let ctx = get_single!(rapier_context);

    for (spring_entity, config, mut timer, mut collider, mut sprite, mut force, spring_transform) in
        spring_query.iter_mut()
    {
        timer.tick(time.delta());

        if timer.just_finished() {
            let mounted_entities = get_mounted_entities(&ctx, spring_entity, spring_transform);

            // Get the calculated launch velocity (fallback to default minimum velocity)
            for entity in mounted_entities {
                if let Ok(mut v) = velocity_query.get_mut(entity) {
                    v.linear.y = 0.0;
                }

                if let Ok(mut impulse) = impulse_query.get_mut(entity) {
                    let mass = mass_query.get(entity).map_or(64.0, |m| m.mass);
                    impulse.impulse.y = force.0.max(mass * 96.0);
                }

                if let Ok(mut transform) = transform_query.get_mut(entity) {
                    transform.translation.y += 2.0;
                }
            }

            // Restore unpressed state
            *collider = Collider::cuboid(4.0, 4.0);
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = config.unpressed_sprite as usize;
            }
        } else if timer.is_finished() {
            // Check for a rigidbody landing on top
            let mounted_entities = get_mounted_entities(&ctx, spring_entity, spring_transform);

            let mounted_entities: Vec<_> = mounted_entities
                .into_iter()
                .filter_map(|e| {
                    if let Ok(v) = velocity_query.get(e)
                        && let Ok(m) = mass_query.get(e)
                    {
                        if v.linear.y <= 0.0 {
                            return Some((m, v));
                        }
                    }
                    None
                })
                .collect();

            if !mounted_entities.is_empty() {
                let len = mounted_entities.len() as f32;
                let (mut mass, mut velocity) = mounted_entities
                    .iter()
                    .fold((0.0, 0.0), |(am, av), (m, v)| {
                        (am + m.get().mass, av + -v.linear.y)
                    });
                mass /= len;
                velocity /= len;

                force.0 = velocity.max(16.0) * (mass * config.strength).sqrt() * 0.90;

                timer.reset();
                *collider = Collider::compound(vec![(
                    Vec2::new(0.0, -2.0),
                    0.0,
                    Collider::cuboid(4.0, 2.0),
                )]);
                if let Some(atlas) = sprite.texture_atlas.as_mut() {
                    atlas.index = config.pressed_sprite as usize;
                }
            }
        }
    }
}

fn get_mounted_entities(
    ctx: &RapierContext<'_>,
    spring_entity: Entity,
    spring_transform: &Transform,
) -> Vec<Entity> {
    let entities = RefCell::new(vec![]);

    let mut center = spring_transform.translation.xy();
    center.y += 3.0;

    let shape = Collider::cuboid(3.5, 2.0);

    let filter = QueryFilter::new()
        .exclude_collider(spring_entity)
        .exclude_sensors();

    ctx.intersect_shape(center, 0.0, (&shape).into(), filter, |e| {
        entities.borrow_mut().push(e);
        true
    });

    entities.take()
}
