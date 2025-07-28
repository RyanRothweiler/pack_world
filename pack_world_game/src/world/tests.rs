mod tests {

    use crate::{drop_table::*, error::*, grid::*, item::*, tile::*, update_signal::*, world::*};
    use elara_engine::{error::Error as EngineError, vectors::*};
    use std::{collections::HashMap, fs::File, io::Write};

    #[cfg(test)]
    pub fn signal_contains_drop(sigs: &Vec<UpdateSignal>, drop_type: DropType) -> bool {
        for sig in sigs {
            match sig {
                UpdateSignal::AddHarvestDrop { drop, origin } => {
                    if drop.drop_type == drop_type {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    // Check that the entire grid points to valid entities. Panic if not.
    #[cfg(test)]
    pub fn validate_grid(world: &World) {
        for (grid, layer) in &world.entity_map {
            for (layer, eid) in &layer.layers {
                let entity = world.get_entity(&eid);
            }
        }
    }

    #[test]
    pub fn insert_overwrite() {
        let mut world = World::new();

        // insert tiles
        let _ = world.insert_tile_type(GridPos::new(0, 0), TileType::Dirt);
        let ret = world.try_place_tile(GridPos::new(1, 0), TileType::Dirt);
        assert!(ret.is_ok());

        // place invalid
        let ret = world.try_place_tile(GridPos::new(10, 10), TileType::Dirt);
        assert!(ret.is_err());

        // place grass on dirt
        let ret = world.try_place_tile(GridPos::new(1, 0), TileType::Grass);
        assert!(ret.is_ok());

        // overwrite grass
        let ret = world
            .try_place_tile(GridPos::new(1, 0), TileType::Grass)
            .unwrap();
        assert_eq!(ret.len(), 2);
        assert!(signal_contains_drop(
            &ret,
            DropType::Item {
                item_type: ItemType::Tile(TileType::Grass),
            },
        ));

        // validate lists
        assert_eq!(world.entities.len(), 3);
        assert_eq!(world.valids.len(), 8);

        // check all valids
        assert_eq!(world.valids.contains_key(&GridPos::new(1, 0)), true);
        assert_eq!(world.valids.contains_key(&GridPos::new(0, 0)), true);

        assert_eq!(world.valids.contains_key(&GridPos::new(1, 1)), true);
        assert_eq!(world.valids.contains_key(&GridPos::new(0, 1)), true);

        assert_eq!(world.valids.contains_key(&GridPos::new(1, -1)), true);
        assert_eq!(world.valids.contains_key(&GridPos::new(0, -1)), true);

        assert_eq!(world.valids.contains_key(&GridPos::new(-1, 0)), true);
        assert_eq!(world.valids.contains_key(&GridPos::new(2, 0)), true);

        validate_grid(&world);
    }

    #[test]
    #[should_panic]
    pub fn tree_invalid_placement() {
        let mut world = World::new();

        let _ = world.insert_tile_type(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(1, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(0, 1), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(1, 1), TileType::Dirt);

        // test invalid placement
        world
            .try_place_tile(GridPos::new(1, 0), TileType::OakTree)
            .unwrap();

        validate_grid(&world);
    }

    #[test]
    pub fn overwrite_tree() {
        let mut world = World::new();

        let _ = world.insert_tile_type(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(1, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(0, 1), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(1, 1), TileType::Dirt);

        world
            .try_place_tile(GridPos::new(0, 0), TileType::OakTree)
            .unwrap();

        let update_sigs = world
            .try_place_tile(GridPos::new(1, 0), TileType::Grass)
            .unwrap();

        assert_eq!(update_sigs.len(), 2);
        assert!(signal_contains_drop(
            &update_sigs,
            DropType::Item {
                item_type: ItemType::Tile(TileType::OakTree),
            },
        ));

        validate_grid(&world);
    }

    // Placing tree. place bird nest in tree. place grass under tree. the bird nest and tree should be given back to user.
    #[test]
    pub fn overwrite_tree_with_nest() {
        let mut world = World::new();

        let _ = world.insert_tile_type(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(1, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(0, 1), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(1, 1), TileType::Dirt);

        world
            .try_place_tile(GridPos::new(0, 0), TileType::OakTree)
            .unwrap();

        world
            .try_place_tile(GridPos::new(0, 0), TileType::BirdNest)
            .unwrap();

        let update_sigs = world
            .try_place_tile(GridPos::new(1, 0), TileType::Grass)
            .unwrap();

        assert_eq!(update_sigs.len(), 3);
        assert!(signal_contains_drop(
            &update_sigs,
            DropType::Item {
                item_type: ItemType::Tile(TileType::OakTree),
            },
        ));
        assert!(signal_contains_drop(
            &update_sigs,
            DropType::Item {
                item_type: ItemType::Tile(TileType::BirdNest),
            },
        ));

        validate_grid(&world);
    }

    #[test]
    pub fn global_drop_count_mod() {
        let mut world = World::new();

        let _ = world.insert_tile_type(GridPos::new(2, 5), TileType::Spring);

        assert_eq!(*world.drop_count_mod.get(&GridPos::new(2, 4)).unwrap(), 2.0);
        assert_eq!(*world.drop_count_mod.get(&GridPos::new(1, 4)).unwrap(), 2.0);

        let _ = world.remove_tile(GridPos::new(2, 5), WorldLayer::Floor);

        assert_eq!(*world.drop_count_mod.get(&GridPos::new(2, 4)).unwrap(), 1.0);
        assert_eq!(*world.drop_count_mod.get(&GridPos::new(1, 4)).unwrap(), 1.0);

        assert!(world.drop_count_mod.get(&GridPos::new(0, 0)).is_none());

        validate_grid(&world);
    }
}
