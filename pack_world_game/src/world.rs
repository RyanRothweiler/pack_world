use crate::{drop_table::*, error::*, grid::*, item::*, tile::*, update_signal::*};
use gengar_engine::{error::Error as EngineError, vectors::*};
use std::{collections::HashMap, fs::File, io::Write};

pub mod entity_id;
pub mod world_cell;
pub mod world_layer;
pub mod world_snapshot;

pub use {entity_id::*, world_cell::*, world_layer::*, world_snapshot::*};

pub struct World {
    /// Get a WorldCell from grid pos.
    pub entity_map: HashMap<GridPos, WorldCell>,

    /// All entities. organized by entity_id
    pub entities: HashMap<EntityID, TileInstance>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<GridPos, bool>,

    pub next_entity_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_map: HashMap::new(),
            valids: HashMap::new(),
            entities: HashMap::new(),
            next_entity_id: 0,
        }
    }

    // Won't place tile if not valid.
    #[must_use]
    pub fn try_place_tile(
        &mut self,
        grid_pos: GridPos,
        tile: TileType,
    ) -> Result<Vec<UpdateSignal>, Error> {
        if !tile.can_place_here(grid_pos, self) {
            return Err(Error::InvalidTilePosition);
        }

        Ok(self.force_insert_tile(grid_pos, tile))
    }

    pub fn get_next_entity_id(&mut self) -> EntityID {
        let ret = EntityID {
            id: self.next_entity_id,
        };
        self.next_entity_id += 1;
        ret
    }

    /// Insert tile
    /// Returns updates signals, because this might need to give new tiles
    #[must_use]
    pub fn force_insert_tile(&mut self, grid_pos: GridPos, tile: TileType) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![UpdateSignal::SaveGame];

        let tile_layer = tile.get_layer();
        let new_entity_id = self.get_next_entity_id();
        let mut inst = tile.create_instance(grid_pos);

        // tell below tiles that something was placed above. They might care.
        let world_cell = self.get_entities(grid_pos);
        for (layer, eid) in &world_cell.layers {
            match self.entities.get_mut(&eid) {
                Some(tile_inst) => tile_inst.methods.tile_placed_ontop(tile, new_entity_id),

                // Nobody there to noify
                None => {}
            };
        }

        // tell the new tile that its been placed
        {
            let mut currents: Vec<&TileInstance> = vec![];
            for (layer, eid) in &world_cell.layers {
                match self.entities.get(&eid) {
                    Some(tile_inst) => currents.push(tile_inst),
                    None => {}
                }
            }

            inst.methods.tile_placed(currents);
        }

        // Find tiles that are going to be overwritten, so that we can give them back to the player
        {
            for p in tile.get_tile_footprint() {
                let pos = grid_pos + p;

                let types_removed = self.remove_tile(pos, tile_layer);
                for t in types_removed {
                    ret.push(UpdateSignal::AddHarvestDrop {
                        drop: Drop::new_tile(t, 1),
                        origin: grid_to_world(&grid_pos),
                    });
                }
            }
        }

        // add new entity
        self.entities.insert(new_entity_id, inst);

        // Add tile to grid map
        for p in tile.get_tile_footprint() {
            let pos = grid_pos + p;

            // update adjacents
            self.valids.insert(pos, true);
            self.valids.insert(GridPos::new(pos.x + 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x - 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x, pos.y + 1), true);
            self.valids.insert(GridPos::new(pos.x, pos.y - 1), true);

            let mut world_cell: &mut WorldCell =
                self.entity_map.entry(pos).or_insert(WorldCell::new());
            world_cell.layers.insert(tile_layer, new_entity_id);
        }

        ret
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.entity_map.clear();
        self.entities.clear();
        self.valids.clear();
        self.next_entity_id = 0;
    }

    /// Used for loading. Just insert the tile without running any global or local state updates
    pub fn raw_insert_entity(&mut self, entity_id: EntityID, tile_instance: TileInstance) {
        let tile_layer = tile_instance.tile_type.get_layer();

        let mut world_cell: &mut WorldCell = self
            .entity_map
            .entry(tile_instance.grid_pos)
            .or_insert(WorldCell::new());
        world_cell.layers.insert(tile_layer, entity_id);

        self.entities.insert(entity_id, tile_instance);
    }

    /// Returns list of tile types removed.
    /// Removing a tile can cause a cascade of multiple tiles removed.
    /// Might return empty if no tiles are removed.
    pub fn remove_tile(
        &mut self,
        pos_removing: GridPos,
        layer_removing: WorldLayer,
    ) -> Vec<TileType> {
        let mut types_removing: Vec<TileType> = vec![];

        let mut eids_removing: Vec<EntityID> = vec![];

        if let Some(world_cell) = self.entity_map.get(&pos_removing) {
            if let Some(eid) = world_cell.layers.get(&layer_removing) {
                if !eids_removing.contains(eid) {
                    eids_removing.push(*eid);
                }
            }
        }

        for eid in eids_removing {
            types_removing.append(&mut self.remove_entity(eid, layer_removing));
        }

        types_removing
    }

    pub fn remove_entity(&mut self, eid: EntityID, layer_removing: WorldLayer) -> Vec<TileType> {
        let mut types_removing: Vec<TileType> = vec![];

        if let Some(tile_inst_removed) = self.entities.remove(&eid) {
            // remove the tile references from the grid map
            for p in tile_inst_removed.tile_type.get_tile_footprint() {
                let pos = tile_inst_removed.grid_pos + p;

                let mut world_cell: &mut WorldCell =
                    self.entity_map.entry(pos).or_insert(WorldCell::new());
                world_cell.layers.remove(&layer_removing);

                types_removing.append(&mut self.remove_invalid(pos));
            }

            // give removed entities back to player
            types_removing.push(tile_inst_removed.tile_type);
        }

        types_removing
    }

    pub fn get_world_snapshot(&self) -> WorldSnapshot {
        let mut ret = WorldSnapshot {
            entity_map: HashMap::new(),
            entities: HashMap::new(),
        };

        ret.entity_map = self.entity_map.clone();

        for (grid_pos, world_layer) in &self.entity_map {
            // for eid in value {
            for (layer_key, eid) in &world_layer.layers {
                let inst: &TileInstance = self.get_entity(eid);
                ret.entities.insert(*eid, inst.methods.into_snapshot());
            }
        }

        ret
    }

    /// Get WorldCell. Will return emtpy world cell if one doesn't exist
    pub fn get_entities(&self, grid_pos: GridPos) -> WorldCell {
        if !self.entity_map.contains_key(&grid_pos) {
            return WorldCell::new();
        }

        return self.entity_map.get(&grid_pos).unwrap().to_owned();
    }

    pub fn cell_contains_type(&self, pos: GridPos, tile_type: TileType) -> bool {
        let world_cell: WorldCell = self.get_entities(pos);
        for (layer, eid) in world_cell.layers {
            let tile = &self.entities.get(&eid).unwrap();

            if tile.tile_type == tile_type {
                return true;
            }
        }

        return false;
    }

    pub fn pos_valid(&self, pos: GridPos) -> bool {
        if !self.entity_map.contains_key(&pos) {
            if !self.valids.contains_key(&pos) {
                return false;
            }
        }
        return true;
    }

    /// Removes tiles that cannot exist where they currently are.
    /// Usually because some other tile was removed that was required.
    /// Returns list of tile_types that were removed.
    pub fn remove_invalid(&mut self, grid_pos: GridPos) -> Vec<TileType> {
        let mut invalid_eids: Vec<(EntityID, WorldLayer)> = vec![];

        if let Some(world_cell) = self.entity_map.get(&grid_pos) {
            for (layer, eid) in &world_cell.layers {
                let tile = &self.entities.get(&eid).unwrap();

                if !tile.tile_type.can_place_here(grid_pos, self) {
                    invalid_eids.push((*eid, *layer));
                }
            }
        }

        let mut ret: Vec<TileType> = vec![];
        for (eid, layer) in invalid_eids {
            ret.append(&mut self.remove_entity(eid, layer));
        }

        ret
    }

    /// Get an entity. Expects the entity to be valid. This is an assumption that must be upheld.
    pub fn get_entity(&self, eid: &EntityID) -> &TileInstance {
        self.entities
            .get(eid)
            .expect(&format!("Invalid entity id {:?}", eid))
    }

    /// Get an entity. Expects the entity to be valid. This is an assumption that must be upheld.
    pub fn get_entity_mut(&mut self, eid: &EntityID) -> &mut TileInstance {
        self.entities
            .get_mut(eid)
            .expect(&format!("Invalid entity id {:?}", eid))
    }
}

mod tests {
    use super::*;

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
        let _ = world.force_insert_tile(GridPos::new(0, 0), TileType::Dirt);
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

        let _ = world.force_insert_tile(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(1, 0), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(0, 1), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(1, 1), TileType::Dirt);

        // test invalid placement
        world
            .try_place_tile(GridPos::new(1, 0), TileType::OakTree)
            .unwrap();

        validate_grid(&world);
    }

    #[test]
    pub fn overwrite_tree() {
        let mut world = World::new();

        let _ = world.force_insert_tile(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(1, 0), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(0, 1), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(1, 1), TileType::Dirt);

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

        let _ = world.force_insert_tile(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(1, 0), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(0, 1), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(1, 1), TileType::Dirt);

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
}
