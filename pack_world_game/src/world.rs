use crate::{drop_table::*, error::*, grid::*, item::*, tile::*, update_signal::*};
use gengar_engine::{change::*, error::Error as EngineError, vectors::*};
use std::{collections::HashMap, fs::File, io::Write};

#[cfg(test)]
pub mod tests;

pub mod entity_id;
pub mod global_mod;
pub mod world_cell;
pub mod world_condition;
pub mod world_condition_state;
pub mod world_layer;
pub mod world_snapshot;

pub use global_mod::*;

pub use {
    entity_id::*, world_cell::*, world_condition::*, world_condition_state::*, world_layer::*,
    world_snapshot::*,
};

/// When placing a tile update all world conditions within this range.
/// Effectively a limit on adjacency ranges
const CONDITIONS_UPDATE_RANGE: i32 = 5;

// Max value of global mod.
const GLOBAL_MOD_MAX: f64 = 100.0;

pub struct World {
    /// Get a WorldCell from grid pos.
    pub entity_map: HashMap<GridPos, WorldCell>,

    /// All entities. organized by entity_id
    pub entities: HashMap<EntityID, TileInstance>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<GridPos, bool>,

    pub next_entity_id: u64,

    // Global drop count modification. Applied to all tiles harvesting at this position
    pub drop_count_mod: HashMap<GridPos, f64>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_map: HashMap::new(),
            valids: HashMap::new(),
            entities: HashMap::new(),
            next_entity_id: 0,
            drop_count_mod: HashMap::new(),
        }
    }

    /// Won't place tile if not valid.
    #[must_use]
    pub fn try_place_tile(
        &mut self,
        grid_pos: GridPos,
        tile: TileType,
    ) -> Result<Vec<UpdateSignal>, Error> {
        if !tile.can_place_here(grid_pos, self) {
            return Err(Error::InvalidTilePosition);
        }

        Ok(self.insert_tile_type(grid_pos, tile))
    }

    pub fn get_next_entity_id(&mut self) -> EntityID {
        let ret = EntityID {
            id: self.next_entity_id,
        };
        self.next_entity_id += 1;
        ret
    }

    /// Create a new tile instance and insert it into the world
    /// Returns updates signals, because this might need to give new tiles
    #[must_use]
    pub fn insert_tile_type(&mut self, grid_pos: GridPos, tile: TileType) -> Vec<UpdateSignal> {
        let mut inst = tile.create_instance(grid_pos);
        self.insert_tile_instance(grid_pos, inst)
    }

    /// Insert tile instance
    /// Returns updates signals, because this might need to give new tiles
    #[must_use]
    pub fn insert_tile_instance(
        &mut self,
        grid_pos: GridPos,
        mut inst: TileInstance,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![UpdateSignal::SaveGame];

        let tile = inst.tile_type;
        let tile_def = tile.get_definition();
        let tile_layer = tile_def.world_layer;

        let new_entity_id = self.get_next_entity_id();

        // tell below tiles that something was placed above. They might care.
        let world_cell = self.get_entities(grid_pos);
        for (layer, eid) in &world_cell.layers {
            match self.entities.get_mut(&eid) {
                Some(tile_inst) => tile_inst.tile_placed_ontop(tile, new_entity_id),

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

            inst.tile_placed(currents);
        }

        // Find tiles that are going to be overwritten, so that we can give them back to the player
        {
            for p in &tile_def.footprint {
                let pos = grid_pos + *p;

                let types_removed = self.remove_tile(pos, tile_layer);
                for t in types_removed {
                    ret.push(UpdateSignal::AddHarvestDrop {
                        drop: Drop::new_tile(t, 1),
                        origin: grid_pos,
                    });
                }
            }
        }

        // add new entity
        self.entities.insert(new_entity_id, inst);

        // Add tile to grid map
        for p in &tile_def.footprint {
            let pos = grid_pos + *p;

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

        // Update world conditions of tiles within range.
        // After insrting the new tile
        self.update_conditions(grid_pos);

        // Update global mods
        for gm in &tile_def.placement_global_mod {
            self.update_global_mod(grid_pos, gm, Change::Adding);
        }

        ret
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.entity_map.clear();
        self.entities.clear();
        self.valids.clear();
        self.drop_count_mod.clear();
        self.next_entity_id = 0;
    }

    /// Update all tile world conditions within the radius
    pub fn update_conditions(&mut self, grid_pos: GridPos) {
        let snapshot = self.get_world_snapshot();

        for pos in grid_pos.to_radius_iter(CONDITIONS_UPDATE_RANGE) {
            let world_cell: WorldCell = self.get_entities(pos);
            for (layer, eid) in world_cell.layers {
                let tile_world = &mut self.entities.get_mut(&eid).unwrap();
                tile_world.update_world_conditions(&snapshot);
            }
        }
    }

    /// Add or remove a global mod.
    pub fn update_global_mod(&mut self, origin: GridPos, global_mod: &GlobalMod, change: Change) {
        let mut positions: Vec<GridPos> = vec![];

        // get all positions
        match global_mod.loc {
            GlobalModLocation::Radius(rad) => {
                let mut p: Vec<GridPos> = origin.to_radius_iter(rad).collect();
                positions.append(&mut p);
            }
        }

        // set the modifications
        match global_mod.kind {
            GlobalModKind::DropCount(drop_mod) => {
                for p in &positions {
                    let mut new_val: f64 = *self.drop_count_mod.get(p).unwrap_or(&1.0);

                    match change {
                        Change::Adding => new_val *= drop_mod.clamp(0.0, GLOBAL_MOD_MAX),
                        Change::Removing => new_val /= drop_mod.clamp(0.0, GLOBAL_MOD_MAX),
                    }

                    self.drop_count_mod.insert(*p, new_val);
                }
            }
        }
    }

    /// Used for loading. Just insert the tile without running any global or local state updates
    pub fn raw_insert_entity(&mut self, entity_id: EntityID, tile_instance: TileInstance) {
        let tile_layer = tile_instance.tile_type.get_definition().world_layer;

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

        // Update world conditions
        self.update_conditions(pos_removing);

        // Update global mods
        for tile_type in &types_removing {
            for gm in &tile_type.get_definition().placement_global_mod {
                self.update_global_mod(pos_removing, gm, Change::Removing);
            }
        }

        types_removing
    }

    /// Remove a tile and ignore the the update signal to add to inventory
    pub fn destroy_tile(&mut self, pos: GridPos, layer: WorldLayer) {
        let _ = self.remove_tile(pos, layer);
    }

    pub fn remove_entity(&mut self, eid: EntityID, layer_removing: WorldLayer) -> Vec<TileType> {
        let mut types_removing: Vec<TileType> = vec![];

        if let Some(tile_inst_removed) = self.entities.remove(&eid) {
            // remove the tile references from the grid map
            for p in &tile_inst_removed.tile_type.get_definition().footprint {
                let pos = tile_inst_removed.grid_pos + *p;

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
            entity_harvest_perc: HashMap::new(),
            valids: self.valids.clone(),
            drop_count_mod: HashMap::new(),
        };

        ret.entity_map = self.entity_map.clone();
        ret.drop_count_mod = self.drop_count_mod.clone();

        for (grid_pos, world_layer) in &self.entity_map {
            // for eid in value {
            for (layer_key, eid) in &world_layer.layers {
                if let Some(tile_inst) = self.entities.get(eid) {
                    ret.entities.insert(*eid, tile_inst.into_snapshot());

                    if let Some(hc) = &tile_inst.comp_harvest {
                        ret.entity_harvest_perc
                            .insert(*grid_pos, (*eid, hc.percent_done()));
                    }
                }
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
