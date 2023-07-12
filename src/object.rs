use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use crate::map::{MAP_HEIGHT, MAP_WIDTH};

/// How many objects will be spawned in the world
const NUM_OBJECTS: u16 = 500;
/// The speed an object moves
const SPEED: f32 = 50.0;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_objects)
            .add_systems(Update, move_along_path)
        ;
    }
}

/// A moving object on the map.
#[derive(Component)]
struct Object {
    path: Path,
}

/// Path an object will move along
#[derive(Copy, Clone)]
struct Path {
    /// The target position
    target: Vec2,
    /// The direction vector to the target from the original position
    direction: Vec2,
}

impl Path {
    fn new(current_position: Vec2, target: Vec2) -> Self {
        Path {
            target,
            direction: (target - current_position).normalize()
        }
    }
}

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
    commands: &mut Commands,
) {
    let start = generate_vec2(rand);
    let target = generate_vec2(rand);

    commands.spawn((
        Object {
            path: Path::new(start, target)
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(MAP_WIDTH / 50.0, MAP_HEIGHT / 50.0)),
                color: Color::BLACK,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::from((start, 1.0))),
            ..default()
        }
    ));
}

fn move_along_path(
    time: Res<Time>,
    mut query: Query<(&mut Object, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    let mut rand = thread_rng();

    for (mut object, mut transform) in &mut query {
        let mut xy = transform.translation.xy();
        let path = object.path;

        xy += path.direction * delta * SPEED;

        if path.target.distance(xy) < 1.0 {
            xy = path.target;
            object.path = Path::new(xy, generate_vec2(&mut rand));
        }

        transform.translation = Vec3::from((xy, transform.translation.z));
    }
}

fn generate_vec2(rand: &mut ThreadRng) -> Vec2 {
    let get_range = |len: f32| (len * 0.025)..(len - len * 0.025);
    let x = rand.gen_range(get_range(MAP_WIDTH));
    let y = rand.gen_range(get_range(MAP_HEIGHT));
    Vec2::new(x, y)
}