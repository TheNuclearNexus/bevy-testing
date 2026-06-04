use std::collections::HashMap;
use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LayerMetadata};
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::tiles::TileColor;
use bevy_rapier2d::prelude::*;

use crate::get_single;
use crate::world::LevelLayer;
use crate::world::foliage::components::{Foliage, FoliageSensor, FoliageSensorBundle};

pub fn spawn_foliage(
    mut commands: Commands,
    layers: Query<&LayerMetadata>,
    foliage: Query<&GridCoords, Added<Foliage>>,
    non_foliage: Query<
        (Entity, &GridCoords, &TilemapId),
        (Without<Foliage>, Without<FoliageSensor>),
    >,
) {
    let mut non_foliage_map = HashMap::<GridCoords, Entity>::new();

    for (entity, coords, tilemap) in non_foliage {
        let Ok(layer) = layers.get(tilemap.0) else {
            continue;
        };

        if layer.identifier != format!("{}_AL", LevelLayer::Foreground) {
            continue;
        }

        non_foliage_map.insert(coords.clone(), entity);
    }

    for coords in foliage {
        let Some(entity) = non_foliage_map.get(coords) else {
            continue;
        };

        commands
            .entity(entity.clone())
            .insert(FoliageSensorBundle::default());
    }
}

pub fn update_foliage_reaction(
    time: Res<Time>,
    rapier_context: ReadRapierContext,
    mut foliage_query: Query<(Entity, &mut TileColor, &mut FoliageSensor)>,
) {
    let ctx = get_single!(rapier_context);

    for (entity, mut tile_color, mut sensor) in foliage_query.iter_mut() {
        let is_intersecting = ctx.intersection_pairs_with(entity).next().is_some();

        if is_intersecting != sensor.intersecting {
            if sensor.timer.is_finished() {
                sensor.timer.reset();
            } else {
                let elapsed = sensor.timer.elapsed_secs();
                let duration = sensor.timer.duration().as_secs_f32();
                sensor.timer.set_elapsed(Duration::from_secs_f32(duration - elapsed));
            }
            sensor.intersecting = is_intersecting;
        }

        tick(&time, &mut tile_color, &mut sensor, is_intersecting);
    }
}

fn tick(
    time: &Time,
    tile_color: &mut TileColor,
    sensor: &mut FoliageSensor,
    is_intersecting: bool,
) {
    if sensor.timer.is_finished() {
        return;
    }

    sensor.timer.tick(time.delta());
    let t = sensor.timer.elapsed_secs() / sensor.timer.duration().as_secs_f32();
    let t = if is_intersecting { 1.0 - t } else { t };

    tile_color.0.set_alpha(0.25.lerp(1.0, t));
}
