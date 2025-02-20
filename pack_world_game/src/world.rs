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

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<GridPos, bool>,

    /// All entities. organized by entity_id
    pub entities: HashMap<EntityID, TileInstance>,

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
        let inst = tile.create_instance(grid_pos);

        // tell below tiles that something was placed above. They might care.
        let world_cell = self.get_entities(grid_pos);
        for (layer, eid) in world_cell.layers {
            match self.entities.get_mut(&eid) {
                Some(tile_inst) => tile_inst.methods.tile_placed_ontop(tile, new_entity_id),

                // Nobody there to noify
                None => {}
            };
        }

        // Find tiles that are going to be overwritte, so that we can give them back to the player
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
                if let Some(tile_instance) = self.entities.remove(&eid) {
                    ret.push(UpdateSignal::AddHarvestDrop {
                        drop: Drop::new_tile(tile_instance.tile_type, 1),
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
            // entities: vec![TileSnapshot::Grass; self.entities.len()],
            entities: HashMap::new(),
        };

        ret.entity_map = self.entity_map.clone();

        for (key, world_layer) in &self.entity_map {
            // for eid in value {
            for (layer_key, eid) in &world_layer.layers {
                let inst: &TileInstance = self.entities.get(eid).expect("Invalid entity id");
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
}
