use bevy::prelude::*;

use game::GamePlugin;
#[cfg(feature = "dev")]
use game::dev::editor::EditorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GamePlugin);

    #[cfg(feature = "dev")]
    app.add_plugins(EditorPlugin);

    app.run();
}
