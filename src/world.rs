use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

pub fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<WallBundle>(2)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_wall_collision);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the levels.ldtk project from assets/platformer/
    let ldtk_handle = asset_server.load("platformer/levels.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: ldtk_handle,
        },
        ..default()
    });

    // Select Level_0 to load
    commands.insert_resource(LevelSelection::Identifier("Level_0".to_string()));
}

/// Spawns wall colliders for the IntGrid wall tiles of the level.
/// Uses a plate-merging algorithm to combine adjacent tiles and minimize physics entities.
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &ChildOf), Added<Wall>>,
    parent_query: Query<(&ChildOf, &LayerMetadata), Without<Wall>>,
) {
    /// Represents a wide wall that is 1 tile tall
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Group wall tile locations by their layer entity (parent)
    let mut layer_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    for (&grid_coords, parent) in &wall_query {
        layer_to_wall_locations
            .entry(parent.get())
            .or_default()
            .insert(grid_coords);
    }

    for (layer_entity, level_walls) in layer_to_wall_locations {
        if let Ok((_grandparent, metadata)) = parent_query.get(layer_entity) {
            let width = metadata.c_wid;
            let height = metadata.c_hei;
            let grid_size = metadata.grid_size;

            // 1. Combine wall tiles into flat "plates" in each individual row
            let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

            for y in 0..height {
                let mut row_plates: Vec<Plate> = Vec::new();
                let mut plate_start = None;

                // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                for x in 0..width + 1 {
                    if level_walls.contains(&GridCoords { x, y }) {
                        if plate_start.is_none() {
                            plate_start = Some(x);
                        }
                    } else if let Some(start) = plate_start {
                        row_plates.push(Plate {
                            left: start,
                            right: x - 1,
                        });
                        plate_start = None;
                    }
                }
                plate_stack.push(row_plates);
            }

            // 2. Combine "plates" into rectangles across multiple rows
            let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
            let mut prev_row: Vec<Plate> = Vec::new();
            let mut wall_rects: Vec<Rect> = Vec::new();

            // An extra empty row so the algorithm "finishes" the rects that touch the top edge
            plate_stack.push(Vec::new());

            for (y, current_row) in plate_stack.into_iter().enumerate() {
                for prev_plate in &prev_row {
                    if !current_row.contains(prev_plate) {
                        // Remove the finished rect so that the same plate in the future starts a new rect
                        if let Some(rect) = rect_builder.remove(prev_plate) {
                            wall_rects.push(rect);
                        }
                    }
                }
                for plate in &current_row {
                    rect_builder
                        .entry(plate.clone())
                        .and_modify(|e| e.top += 1)
                        .or_insert(Rect {
                            bottom: y as i32,
                            top: y as i32,
                            left: plate.left,
                            right: plate.right,
                        });
                }
                prev_row = current_row;
            }

            // 3. Spawn colliders as children of the layer entity
            commands.entity(layer_entity).with_children(|level| {
                for wall_rect in wall_rects {
                    let rect_width =
                        (wall_rect.right - wall_rect.left + 1) as f32 * grid_size as f32;
                    let rect_height =
                        (wall_rect.top - wall_rect.bottom + 1) as f32 * grid_size as f32;

                    let center_x =
                        (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.0;
                    let center_y =
                        (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.0;

                    level
                        .spawn_empty()
                        .insert(Collider::cuboid(rect_width / 2.0, rect_height / 2.0))
                        .insert(RigidBody::Fixed)
                        .insert(Friction::new(1.0))
                        .insert(Transform::from_xyz(center_x - 4.0, center_y - 4.0, 0.0))
                        .insert(GlobalTransform::default());
                }
            });
        }
    }
}
