use bevy::{camera::ScalingMode, prelude::*};
#[cfg(feature = "dev")]
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use bevy_rapier2d::plugin::PhysicsSet;
#[cfg(feature = "dev")]
use transform_gizmo_bevy::GizmoCamera;

use crate::entity::player::PlayerConfig;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(PostUpdate, update.after(PhysicsSet::Writeback));
}

#[derive(Component)]
pub struct GameCamera;

fn setup(mut commands: Commands) {
    // 1. Spawn a 2D Camera, positioned to view the loaded LDTK level
    let mut camera = commands.spawn((
        Name::new("Main Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.0 / 4.0,
            ..OrthographicProjection::default_2d()
        }),
        GameCamera,
        Transform::from_xyz(250.0, 200.0, 0.0),
    ));

    #[cfg(feature = "dev")]
    camera.insert((GizmoCamera, PrimaryEguiContext));
}

fn update(
    mut camera: Single<&mut Transform, With<GameCamera>>,
    players: Query<&Transform, (With<PlayerConfig>, Without<GameCamera>)>,
) {
    let mut average_pos = Vec3::default();

    for player in players.iter() {
        average_pos += player.translation;
    }

    camera.translation = average_pos;
}
