use crate::{drop_table::*, error::*, grid::*, item::*, tile::*, update_signal::*};
use gengar_engine::vectors::*;
use std::collections::HashMap;

pub mod world_snapshot;
pub use world_snapshot::*;

#[derive(Clone, Hash, Eq, PartialEq)]
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

#[derive(Clone)]
pub struct WorldCell {
    pub layers: HashMap<WorldLayer, usize>,
}

impl WorldCell {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
        }
    }
}

pub struct World {
    pub entity_map: HashMap<GridPos, WorldCell>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<GridPos, bool>,

    pub entities: Vec<TileInstance>,
}

impl World {
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

    /// Insert tile
    /// Returns updates signals, because this might need to give new tiles
    #[must_use]
    pub fn force_insert_tile(&mut self, grid_pos: GridPos, tile: TileType) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let inst_id = self.entities.len();
        let inst = tile.create_instance(grid_pos);

        // tell below tiles that something was placed above. They might care.
        let world_cell = self.get_entities(grid_pos);
        for (layer, eid) in world_cell.layers {
            match self.entities.get_mut(eid) {
                Some(tile_inst) => tile_inst.methods.tile_placed_ontop(tile, inst_id),

                // Nobody there to noify
                None => {}
            };
        }

        self.entities.push(inst);

        for p in tile.get_tile_footprint() {
            let pos = grid_pos + p;

            // update adjacents
            self.valids.insert(pos, true);
            self.valids.insert(GridPos::new(pos.x + 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x - 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x, pos.y + 1), true);
            self.valids.insert(GridPos::new(pos.x, pos.y - 1), true);

            //TODO check if there is already something in that layer
            // if a tile is getting overwritten then create a drop for it
            /*
            ret.push(UpdateSignal::AddHarvestDrop {
                drop: drop_table.get_drop(),
                origin: grid_to_world(&grid_pos),
            });
            */

            // .push(inst_id)
            let layer_type = tile.get_layer();
            let mut world_cell: &mut WorldCell =
                self.entity_map.entry(pos).or_insert(WorldCell::new());
            world_cell.layers.insert(layer_type, inst_id);
        }

        ret
    }

    pub fn get_world_snapshot(&self) -> WorldSnapshot {
        let mut ret = WorldSnapshot {
            entity_map: HashMap::new(),
            entities: vec![TileSnapshot::Grass; self.entities.len()],
        };

        ret.entity_map = self.entity_map.clone();

        for (key, world_layer) in &self.entity_map {
            // for eid in value {
            for (layer_key, eid) in &world_layer.layers {
                let inst: &TileInstance = self.entities.get(*eid).unwrap();
                ret.entities[*eid] = inst.methods.into_snapshot();
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
            let tile = &self.entities[eid];

            if tile.tile_type == tile_type {
                return true;
            }
        }

        return false;
    }
}
