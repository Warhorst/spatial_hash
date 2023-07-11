use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use crate::map::{MAP_HEIGHT, MAP_WIDTH};

/// How many objects will be spawned in the world
const NUM_OBJECTS: u8 = 100;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_objects)
        ;
    }
}

/// Marker for an object on the map.
#[derive(Component)]
struct Object;

fn spawn_objects(
    mut commands: Commands
) {
    let mut rand = thread_rng();

    for _ in 0..NUM_OBJECTS {
        spawn_object(&mut rand, &mut commands);
    }
}

fn spawn_object(
    rand: &mut ThreadRng,
    commands: &mut Commands
) {
    let x = rand.gen_range(MAP_WIDTH / 10.0..MAP_WIDTH - MAP_WIDTH / 10.0);
    let y = rand.gen_range(MAP_HEIGHT / 10.0..MAP_HEIGHT - MAP_HEIGHT / 10.0);

    commands.spawn((
        Object,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(MAP_WIDTH / 50.0, MAP_HEIGHT / 50.0)),
                color: Color::GRAY,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..default()
        }
    ));
}