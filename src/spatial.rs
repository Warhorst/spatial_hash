use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use pad::{Position, p};
use crate::map::{MAP_HEIGHT, MAP_WIDTH};
use crate::object::Object;

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_spatial_hash_grid)
            .add_systems(Update, update_spatial_hash_grid)
        ;
    }
}

fn create_spatial_hash_grid(
    mut commands: Commands
) {
    commands.insert_resource(SpatialHashGrid::new(
        (Vec2::default(), Vec2::new(MAP_WIDTH, MAP_HEIGHT)),
        (100, 100)
    ));
}

/// Update the spatial hash of all objects in the world. If an object was not
/// yet added to the spatial hash, this will happen here.
///
/// Note: To keep things simple, the Spatial struct is not a component. If it was,
/// it would be necessary to keep it in sync with the sprite position and dimension.
fn update_spatial_hash_grid(
    mut grid: ResMut<SpatialHashGrid>,
    query: Query<(Entity, &Transform, &Sprite), With<Object>>
) {
    for (entity, transform, sprite) in &query {
        let coordinates = transform.translation.xy();
        let size = sprite.custom_size.expect("custom size must be set");
        let spatial = Spatial {
            coordinates,
            dimension: Dimension {
                width: size.x,
                height: size.y
            }
        };

        grid.update_entity(entity, spatial);
    }
}

/// The spatial properties of an entity.
pub struct Spatial {
    pub coordinates: Vec2,
    pub dimension: Dimension,
}

/// The dimension of an entity, aka how big it is in the world
pub struct Dimension {
    pub width: f32,
    pub height: f32,
}

/// Divides the game/simulation map into cells. Every entity with a position and size can
/// be assigned to one or more cells. Entities at a specific position can be fastly retrieved.
#[derive(Resource)]
pub struct SpatialHashGrid {
    /// The bottom left and top right corner points of the map.
    bounds: (Vec2, Vec2),
    /// The dimension of the cells. For example: (10, 10) means 10x10 cells
    dimensions: (usize, usize),
    /// Mapping from a position to a set of entities at this position
    position_entities_map: HashMap<Position, HashSet<Entity>>,
    /// Mapping from an entity to a pair of corner positions. For example: If an entity
    /// touches the positions (1,1), (1,2), (2,1) and (2,2), the mapping will hold the tuple ((1,1), (2,2))
    entity_indices_map: HashMap<Entity, (Position, Position)>
}

impl SpatialHashGrid {
    pub fn new(
        bounds: (Vec2, Vec2),
        dimensions: (usize, usize),
    ) -> Self {
        Self {
            bounds,
            dimensions,
            position_entities_map: HashMap::new(),
            entity_indices_map: HashMap::new()
        }
    }

    /// Insert an entity with given spatial properties into the grid. If it already exists,
    /// it will be overwritten.
    pub fn insert_entity(&mut self, entity: Entity, spatial: Spatial) {
        let Vec2 { x, y } = spatial.coordinates;
        let Dimension { width, height } = spatial.dimension;

        let pos_1 = self.get_cell_index(Vec2::new(x - width / 2.0, y - height / 2.0));
        let pos_2 = self.get_cell_index(Vec2::new(x + width / 2.0, y + height / 2.0));

        self.entity_indices_map.insert(entity, (pos_1, pos_2));

        for pos in pos_1.iter_to(pos_2) {
            if !self.position_entities_map.contains_key(&pos) {
                self.position_entities_map.insert(pos, HashSet::new());
            }

            self.position_entities_map.get_mut(&pos).expect("set should exist").insert(entity);
        }
    }

    fn get_cell_index(&self, pos: Vec2) -> Position {
        let Vec2 { x, y } = pos;
        let x = sat((x - self.bounds.0.x) / (self.bounds.1.x - self.bounds.0.x));
        let y = sat((y - self.bounds.0.y) / (self.bounds.1.y - self.bounds.0.y));

        let x_index = (x * (self.dimensions.0 - 1) as f32).floor() as usize;
        let y_index = (y * (self.dimensions.1 - 1) as f32).floor() as usize;

        p!(x_index, y_index)
    }

    /// Update an entities spatial hash with the new spatial properties.
    pub fn update_entity(&mut self, entity: Entity, spatial: Spatial) {
        self.remove_entity(entity);
        self.insert_entity(entity, spatial);
    }

    /// Remove an entity from the spatial hash. If it does not exist, nothing will happen.
    pub fn remove_entity(&mut self, entity: Entity) {
        let (pos_1, pos_2) = match self.entity_indices_map.get(&entity) {
            Some((pos_1, pos_2)) => (pos_1, pos_2),
            None => return
        };

        for pos in pos_1.iter_to(*pos_2) {
            self.position_entities_map.get_mut(&pos).expect("set should exist").remove(&entity);
        }

        self.entity_indices_map.remove(&entity);
    }

    /// Return all entities which touch the cells at the given positions
    pub fn get_entities_at_positions(
        &self,
        positions: impl IntoIterator<Item=Position>
    ) -> impl IntoIterator<Item=&Entity> {
        positions
            .into_iter()
            .flat_map(|pos| self.position_entities_map.get(&pos))
            .flatten()
    }
}

// TODO what does 'sat' stand for?
fn sat(x: f32) -> f32 {
    f32::min(f32::max(x, 0.0), 1.0)
}