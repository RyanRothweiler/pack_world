use crate::{drop_table::*, error::*, grid::*, tile::*};
use gengar_engine::vectors::*;
use std::collections::HashMap;

pub struct World {
    pub entity_map: HashMap<GridPos, Vec<usize>>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<GridPos, bool>,

    pub entities: Vec<TileInstance>,
}

impl World {
    // Won't place tile if not valid.
    pub fn try_place_tile(&mut self, grid_pos: GridPos, tile: TileType) -> Result<(), Error> {
        if !tile.can_place_here(grid_pos, self) {
            return Err(Error::InvalidTilePosition);
        }

        self.force_insert_tile(grid_pos, tile);

        return Ok(());
    }

    pub fn force_insert_tile(&mut self, grid_pos: GridPos, tile: TileType) {
        let inst_id = self.entities.len();
        let inst = tile.create_instance(grid_pos);
        self.entities.push(inst);

        for p in tile.get_tile_footprint() {
            let pos = grid_pos + p;

            // update adjacents
            self.valids.insert(pos, true);
            self.valids.insert(GridPos::new(pos.x + 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x - 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x, pos.y + 1), true);
            self.valids.insert(GridPos::new(pos.x, pos.y - 1), true);

            self.entity_map.entry(pos).or_insert(vec![]).push(inst_id);
        }

        // this super stucks. we need to figure out something better
        // update adjacent tile state
        match tile {
            TileType::BirdNest => {
                for adj_pos in grid_pos.to_adjacents_iter() {
                    println!("{:?}", adj_pos);

                    let adj_entity_ids = self.get_entities(adj_pos).unwrap_or(vec![]);

                    for adj_entity_id in adj_entity_ids {
                        let adj_tile_inst = self
                            .entities
                            .get_mut(adj_entity_id)
                            .expect("Invalid entity id");

                        match &mut adj_tile_inst.methods {
                            TileMethods::Grass(tile_state) => {
                                tile_state.drop_table = DropTableID::Boulder;
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_entities(&self, grid_pos: GridPos) -> Option<Vec<usize>> {
        if !self.entity_map.contains_key(&grid_pos) {
            return None;
        }

        return Some(self.entity_map.get(&grid_pos).unwrap().to_owned());
    }
}
