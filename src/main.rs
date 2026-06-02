use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
