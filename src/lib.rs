use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;

mod camera;
#[cfg(feature = "dev")]
pub mod dev;
mod entity;
mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0));

        #[cfg(feature = "dev")]
        app.add_plugins((
            EguiPlugin::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
        ))
        .add_systems(Update, update);

        app.add_plugins((entity::plugin, world::plugin, camera::plugin));
    }
}

#[cfg(feature = "dev")]
fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_context: ResMut<DebugRenderContext>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        debug_context.enabled = !debug_context.enabled;
    }
}

#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.single() {
            Ok(m) => m,
            #[cfg(debug_assertions)]
            _ => {
                panic!("Attempted to get_single but failed");
            }
            #[cfg(not(debug_assertions))]
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! defaults {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $fvis:vis $fname:ident : $fty:ident $(= $fdefault:expr)?
            ),* $(,)?
        }
    ) => {
        // Outputs the clean, raw struct definition
        $(#[$struct_meta])*
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $fvis $fname : $fty,
            )*
        }

        // Implements Default manually without ..Default::default() recursion
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $fname : defaults!(@value $($fdefault)?),
                    )*
                }
            }
        }
    };

    (@value) => { Default::default() };
    (@value $val:expr) => { $val };
}
