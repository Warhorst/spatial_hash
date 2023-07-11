use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::map::{MAP_HEIGHT, MAP_WIDTH, MapPlugin};

mod spatial;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800.0, 600.0).into(),
                        title: "spatial_hash".to_string(),
                        resizable: false,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(MapPlugin)
        .add_systems(Startup, spawn_camera)
        .run()
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 2.0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                MAP_WIDTH / 2.0,
                MAP_HEIGHT / 2.0,
                1000.0
            )),
            ..default()
        }
    );
}
