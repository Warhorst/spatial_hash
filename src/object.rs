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

#[derive(Copy, Clone, Debug)]
struct Path {
    a: Vec2,
    b: Vec2,
    target: Vec2,
    direction: Vec2,
}

impl Path {
    fn new(a: Vec2, b: Vec2) -> Self {
        Path {
            a,
            b,
            target: b,
            direction: (b - a).normalize(),
        }
    }

    fn reverse(&mut self) {
        self.direction = -1.0 * self.direction;

        self.target = match self.target == self.a {
            true => self.b,
            false => self.a
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
    let path = generate_path(rand);

    commands.spawn((
        Object {
            path
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(MAP_WIDTH / 50.0, MAP_HEIGHT / 50.0)),
                color: Color::BLACK,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::from((path.a, 1.0))),
            ..default()
        }
    ));
}

fn generate_path(rand: &mut ThreadRng) -> Path {
    let get_range = |len: f32| (len * 0.025)..(len - len * 0.025);

    let xa = rand.gen_range(get_range(MAP_WIDTH));
    let ya = rand.gen_range(get_range(MAP_HEIGHT));
    let xb = rand.gen_range(get_range(MAP_WIDTH));
    let yb = rand.gen_range(get_range(MAP_HEIGHT));

    Path::new(
        Vec2::new(xa, ya),
        Vec2::new(xb, yb),
    )
}

fn move_along_path(
    time: Res<Time>,
    mut query: Query<(&mut Object, &mut Transform)>,
) {
    let delta = time.delta_seconds();

    for (mut object, mut transform) in &mut query {
        let mut xy = transform.translation.xy();
        let path = object.path;

        xy += path.direction * delta * SPEED;

        if path.target.distance(xy) < 1.0 {
            xy = path.target;
            object.path.reverse();
        }

        transform.translation = Vec3::from((xy, transform.translation.z));
    }
}