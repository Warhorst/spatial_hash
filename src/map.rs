use bevy::prelude::*;
use bevy::sprite::Anchor;

pub const MAP_WIDTH: f32 = 1000.0;
pub const MAP_HEIGHT: f32 = 1000.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_map)
        ;
    }
}

fn spawn_map(
    mut commands: Commands
) {
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(MAP_WIDTH, MAP_HEIGHT)),
                color: Color::BEIGE,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::default()),
            ..default()
        }
    );
}