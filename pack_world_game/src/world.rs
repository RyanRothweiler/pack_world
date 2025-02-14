use crate::{drop_table::*, error::*, grid::*, item::*, tile::*};
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
                // find the base tree.
                // this assumes the tree_origin is the top left.
                // I'm sure this will result in a bug one day
                let mut tree_origin: GridPos = GridPos::new(0, 0);
                let mut found = false;
                let pos_entities_index = self.get_entities(grid_pos).unwrap();
                for p in pos_entities_index {
                    let tile_instance = self.entities.get(p).unwrap();
                    if tile_instance.tile_type == TileType::OakTree {
                        tree_origin = tile_instance.grid_pos;
                        found = true;
                    }
                }
                if !found {
                    panic!("There must always be a tree for this bird nest.");
                }

                // this assumes the tree footprint of four grid spaces
                let adj_offsets: Vec<GridPos> = vec![
                    GridPos::new(-1, -1),
                    GridPos::new(0, -1),
                    GridPos::new(1, -1),
                    GridPos::new(2, -1),
                    //
                    GridPos::new(-1, 0),
                    GridPos::new(2, 0),
                    //
                    GridPos::new(-1, 1),
                    GridPos::new(2, 1),
                    //
                    GridPos::new(-1, 2),
                    GridPos::new(0, 2),
                    GridPos::new(1, 2),
                    GridPos::new(2, 2),
                ];

                // go through the adjacents for that tree
                for offset in adj_offsets {
                    let adj_pos = tree_origin + offset;

                    let adj_entity_ids = self.get_entities(adj_pos).unwrap_or(vec![]);

                    for adj_entity_id in adj_entity_ids {
                        let adj_tile_inst = self
                            .entities
                            .get_mut(adj_entity_id)
                            .expect("Invalid entity id");

                        match &mut adj_tile_inst.methods {
                            TileMethods::Grass(tile_state) => {
                                tile_state
                                    .harvest_timer
                                    .add_entry((EntryOutput::new_item(ItemType::Acorn, 1), 2.0));
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
