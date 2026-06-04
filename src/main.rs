use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use game::GamePlugin;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
        )
        .add_plugins(GamePlugin);

    #[cfg(feature = "dev")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}
