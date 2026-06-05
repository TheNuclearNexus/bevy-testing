use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod common;
pub mod foliage;
pub mod platform;
pub mod vines;
pub mod wall;

use bevy_rapier2d::geometry::Group as RapierGroup;

use foliage::systems::update_foliage_reaction;
use wall::components::WallBundle;
use wall::systems::spawn_wall_collision;

use crate::world::foliage::components::FoliageBundle;
use crate::world::foliage::systems::spawn_foliage;
use crate::world::platform::components::{Platform, PlatformBundle};
use crate::world::vines::components::VinesBundle;

pub struct Group;

impl Group {
    pub const GRABBABLE: RapierGroup = RapierGroup::GROUP_1;

    pub fn empty() -> RapierGroup {
        RapierGroup::empty()
    }

    pub fn all() -> RapierGroup {
        RapierGroup::all()
    }
}

#[allow(dead_code)]
pub enum LevelLayer {
    Background,
    Collision,
    Foreground,
}

impl std::fmt::Display for LevelLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelLayer::Background => write!(f, "Background"),
            LevelLayer::Collision => write!(f, "Collision"),
            LevelLayer::Foreground => write!(f, "Foreground"),
        }
    }
}

fn register_cell<B: Bundle + LdtkIntCell>(app: &mut App, layer: LevelLayer, cell: Option<i32>) {
    match cell {
        Some(cell) => {
            app.register_ldtk_int_cell_for_layer::<B>(&layer.to_string(), cell);
        }
        None => {
            app.register_default_ldtk_int_cell_for_layer::<B>(&layer.to_string());
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        /* -- Register Types -- */
        // Platform
        .register_type::<Platform>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (spawn_wall_collision, spawn_foliage, update_foliage_reaction),
        );

    register_cell::<WallBundle>(app, LevelLayer::Collision, None);
    register_cell::<PlatformBundle>(app, LevelLayer::Collision, Some(3));
    register_cell::<VinesBundle>(app, LevelLayer::Collision, Some(4));
    
    register_cell::<FoliageBundle>(app, LevelLayer::Foreground, Some(1));
    register_cell::<FoliageBundle>(app, LevelLayer::Foreground, Some(2));

    #[cfg(feature = "dev")]
    app.add_systems(Update, reload_level);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

#[cfg(feature = "dev")]
pub fn reload_level(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<LevelIid>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for entity in &query {
            commands.entity(entity).insert(Respawn);
        }
    }
}
