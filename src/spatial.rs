use bevy::prelude::*;
use bevy::utils::HashMap;
use pad::Position;

/// The exact position of an entity.
pub struct Coordinates(Vec2);

/// The dimension of an entity, aka how big it is in the world
pub struct Dimension {
    pub width: f32,
    pub height: f32
}

#[derive(Component)]
pub struct Spatial {
    pub coordinates: Coordinates,
    pub dimension: Dimension
}

// https://github.com/simondevyoutube/Tutorial_SpatialHashGrid_Optimized
pub struct SpacialHash {
    position_entities_map: HashMap<Position, Vec<Entity>>
}

impl SpacialHash {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn insert_entity(&mut self, entity: Entity, spatial: Spatial) {
        unimplemented!()
    }

    pub fn update_entity(&mut self, entity: Entity, spatial: Spatial) {
        unimplemented!()
    }

    pub fn remove_entity(&mut self) {
        unimplemented!()
    }

    pub fn get_entities_on_position(&self, pos: Position) -> Vec<Entity> {
        unimplemented!()
    }
}