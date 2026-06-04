use bevy::picking::backend::{HitData, PointerHits, ray::RayMap};
use bevy::picking::{Pickable, PickingSystems};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct RapierPickingPlugin;

impl Plugin for RapierPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_hits.in_set(PickingSystems::Backend));
    }
}

pub fn update_hits(
    ray_map: Res<RayMap>,
    picking_cameras: Query<(&Camera, &GlobalTransform)>,
    pickables: Query<&Pickable>,
    transforms: Query<&GlobalTransform>,
    rapier_context: ReadRapierContext,
    mut pointer_hits_writer: MessageWriter<PointerHits>,
) {
    let ctx = crate::get_single!(rapier_context);

    for (&ray_id, &ray) in ray_map.iter() {
        let Ok((camera, camera_transform)) = picking_cameras.get(ray_id.camera) else {
            continue;
        };

        // Project the 3D ray onto the 2D plane (Z = 0)
        let plane_normal = Vec3::Z;
        let denominator = ray.direction.dot(plane_normal);
        if denominator.abs() < 1e-6 {
            continue;
        }
        let t = -ray.origin.dot(plane_normal) / denominator;
        if t < 0.0 {
            continue;
        }
        let intersection_point = ray.origin + *ray.direction * t;
        let point_2d = intersection_point.xy();

        let mut picks = Vec::new();
        let filter = QueryFilter::default();

        ctx.intersect_point(point_2d, filter, |entity| {
            // Check if the entity is hoverable/pickable
            let is_pickable = pickables.get(entity).map_or(true, |p| p.is_hoverable);
            if is_pickable {
                // Determine Z-distance from camera to the entity
                let distance = if let Ok(transform) = transforms.get(entity) {
                    camera_transform.translation().z - transform.translation().z
                } else {
                    t
                };

                let hit_data = HitData::new(
                    ray_id.camera,
                    distance,
                    Some(intersection_point),
                    Some(Vec3::Z),
                );
                picks.push((entity, hit_data));
            }
            true
        });

        let order = camera.order as f32;
        if !picks.is_empty() {
            pointer_hits_writer.write(PointerHits::new(ray_id.pointer, picks, order));
        }
    }
}
