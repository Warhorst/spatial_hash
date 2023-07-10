use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use pad::{Position, p};

/// The dimension of an entity, aka how big it is in the world
pub struct Dimension {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct Spatial {
    pub coordinates: Vec2,
    pub dimension: Dimension,
}

// https://github.com/simondevyoutube/Tutorial_SpatialHashGrid_Optimized
pub struct SpatialHash {
    bounds: (Vec2, Vec2),
    dimensions: (usize, usize),
    position_entities_map: HashMap<Position, HashSet<Entity>>,
    entity_indices_map: HashMap<Entity, (Position, Position)>
}

impl SpatialHash {
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

    pub fn update_entity(&mut self, entity: Entity, spatial: Spatial) {
        self.remove_entity(entity);
        self.insert_entity(entity, spatial);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        let (pos_1, pos_2) = self.entity_indices_map.get(&entity).expect("the entity should exists in the map");

        for pos in pos_1.iter_to(*pos_2) {
            self.position_entities_map.get_mut(&pos).expect("set should exist").remove(&entity);
        }

        self.entity_indices_map.remove(&entity);
    }

    pub fn get_entities_on_position(&self, pos: Position) -> impl IntoIterator<Item=&Entity> {
        self.position_entities_map.get(&pos).expect("set should exist").iter()
    }
}

// TODO what does 'sat' stand for?
fn sat(x: f32) -> f32 {
    f32::min(f32::max(x, 0.0), 1.0)
}