use crate::{drop_table::*, error::*, grid::*, item::*, tile::*, update_signal::*};
use gengar_engine::vectors::*;
use std::collections::HashMap;

pub mod world_snapshot;
pub use world_snapshot::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum WorldLayer {
    /// The ground itself. Dirt, water, sand.
    Ground,

    /// Stuff on the floor. Trees, grass.
    Floor,

    /// Find something better to do here. This is just for the birds nest. What to do about attachments?
    TreeAttachment,
}

impl WorldLayer {
    pub fn to_index(&self) -> i32 {
        match self {
            WorldLayer::Ground => 0,
            WorldLayer::Floor => 1,
            WorldLayer::TreeAttachment => 2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorldCell {
    pub layers: HashMap<WorldLayer, EntityID>,
}

impl WorldCell {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub struct EntityID {
    id: u64,
}

pub struct World {
    /// Get a WorldCell from grid pos.
    pub entity_map: HashMap<GridPos, WorldCell>,

    /// All entities. organized by entity_id
    pub entities: HashMap<EntityID, TileInstance>,

    // valid positions, and all adjacent valid positions
    valids: HashMap<GridPos, bool>,

    next_entity_id: u64,
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
        let mut ret: Vec<UpdateSignal> = vec![];

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
            let mut eids_removing: Vec<EntityID> = vec![];

            for p in tile.get_tile_footprint() {
                let pos = grid_pos + p;

                let mut world_cell: &mut WorldCell =
                    self.entity_map.entry(pos).or_insert(WorldCell::new());

                if world_cell.layers.contains_key(&tile_layer) {
                    let eid = world_cell.layers.get(&tile_layer).unwrap();
                    if !eids_removing.contains(eid) {
                        eids_removing.push(*eid);
                    }
                }
            }

            for eid in eids_removing {
                if let Some(tile_inst_removed) = self.entities.remove(&eid) {
                    // remove the tile references from the grid map
                    for p in tile_inst_removed.tile_type.get_tile_footprint() {
                        let pos = tile_inst_removed.grid_pos + p;

                        let mut world_cell: &mut WorldCell =
                            self.entity_map.entry(pos).or_insert(WorldCell::new());
                        world_cell.layers.remove(&tile_layer);
                    }

                    // give removed entities back to player
                    ret.push(UpdateSignal::AddHarvestDrop {
                        drop: Drop::new_tile(tile_inst_removed.tile_type, 1),
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

            // .push(inst_id)
            let mut world_cell: &mut WorldCell =
                self.entity_map.entry(pos).or_insert(WorldCell::new());
            world_cell.layers.insert(tile_layer, new_entity_id);
        }

        ret
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

    pub fn cell_contains_tile(&self, pos: GridPos, tile_type: TileType) -> bool {
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

    /// Get an entity. Expects the entity to be valid. This is an assumption that must be upheld.
    pub fn get_entity(&self, eid: &EntityID) -> &TileInstance {
        self.entities.get(eid).expect("Invalid entity id")
    }

    /// Get an entity. Expects the entity to be valid. This is an assumption that must be upheld.
    pub fn get_entity_mut(&mut self, eid: &EntityID) -> &mut TileInstance {
        self.entities.get_mut(eid).expect("Invalid entity id")
    }
}

mod tests {
    use super::*;

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
        let ret = world.try_place_tile(GridPos::new(1, 0), TileType::Grass);
        match ret {
            Ok(list) => {
                assert_eq!(list.len(), 1);
                match list.get(0).unwrap() {
                    UpdateSignal::AddHarvestDrop { drop, origin } => {
                        assert_eq!(
                            drop.drop_type,
                            DropType::Item {
                                item_type: ItemType::Tile(TileType::Grass),
                            }
                        );
                    }
                    _ => {
                        panic!(
                            "Overwriting should give a harvest drop to return the item harvested"
                        );
                    }
                }
            }
            Err(e) => {
                panic!("Should be a valid placement.");
            }
        }

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
    }
}
