use crate::{tile_type::TileType, world::*};

/// A 'world query'.
/// Used to check if something is true about the world.

#[derive(Debug)]
pub enum WorldCondition {
    /// Adjacent to a specific tile type
    /// False if origin is the tile type. Origin does not count as adjacent to
    AdjacentTo(TileSnapshot),

    /// Checks if the origin world cell contains a tile type
    OriginContains(TileSnapshot),

    /// Check the valids hashmap. For adjacency
    ValidPosition(),
}

impl WorldCondition {
    pub fn valid(&self, origin: GridPos, world_snapshot: &WorldSnapshot) -> bool {
        match self {
            Self::AdjacentTo(ty) => {
                for adj_pos in origin.to_adjacents_iter() {
                    if let Some(world_cell) = world_snapshot.entity_map.get(&adj_pos) {
                        for (layer, eid) in &world_cell.layers {
                            let tile = world_snapshot.entities.get(&eid).unwrap();
                            if *tile == *ty {
                                return true;
                            }
                        }
                    }
                }
            }

            Self::OriginContains(ty) => {
                if let Some(world_cell) = world_snapshot.entity_map.get(&origin) {
                    for (layer, eid) in &world_cell.layers {
                        let tile = world_snapshot.entities.get(&eid).unwrap();
                        if *tile == *ty {
                            return true;
                        }
                    }
                }
            }

            Self::ValidPosition() => {
                if !world_snapshot.entity_map.contains_key(&origin) {
                    if !world_snapshot.valids.contains_key(&origin) {
                        return false;
                    }
                }
                return true;
            }
        }

        return false;
    }
}

mod test {
    use super::*;

    #[test]
    fn adjacent_to() {
        let mut world = World::new();

        let _ = world.insert_tile_type(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(0, 1), TileType::Water);
        let _ = world.insert_tile_type(GridPos::new(10, 10), TileType::Grass);

        let snapshot = world.get_world_snapshot();

        assert_eq!(
            WorldCondition::AdjacentTo(TileSnapshot::Water).valid(GridPos::new(0, 0), &snapshot),
            true
        );

        assert_eq!(
            WorldCondition::AdjacentTo(TileSnapshot::Water).valid(GridPos::new(10, 10), &snapshot),
            false
        );

        assert_eq!(
            WorldCondition::AdjacentTo(TileSnapshot::Water).valid(GridPos::new(0, 1), &snapshot),
            false
        );

        assert_eq!(
            WorldCondition::AdjacentTo(TileSnapshot::Water).valid(GridPos::new(0, 2), &snapshot),
            true
        );

        assert_eq!(
            WorldCondition::AdjacentTo(TileSnapshot::Water).valid(GridPos::new(-1, 1), &snapshot),
            true
        );
    }

    #[test]
    fn origin_contains() {
        let mut world = World::new();

        let _ = world.insert_tile_type(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(0, 1), TileType::Water);
        let _ = world.insert_tile_type(GridPos::new(10, 10), TileType::Grass);

        let snapshot = world.get_world_snapshot();

        assert_eq!(
            WorldCondition::OriginContains(TileSnapshot::Water)
                .valid(GridPos::new(0, 0), &snapshot),
            false
        );

        assert_eq!(
            WorldCondition::OriginContains(TileSnapshot::Water)
                .valid(GridPos::new(0, 1), &snapshot),
            true
        );

        assert_eq!(
            WorldCondition::OriginContains(TileSnapshot::Grass)
                .valid(GridPos::new(5, 5), &snapshot),
            false
        );

        assert_eq!(
            WorldCondition::OriginContains(TileSnapshot::Grass)
                .valid(GridPos::new(10, 10), &snapshot),
            true
        );
    }
}
