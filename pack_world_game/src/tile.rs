use crate::{
    grid::*,
    state::{assets::*, *},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    vectors::*,
};
use std::collections::HashMap;

pub mod harvest_timer;
pub mod tiles;

use tiles::{tile_bird_nest::*, tile_boulder::*, tile_dirt::*, tile_grass::*, tile_oak_tree::*};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum TileType {
    Dirt,
    Grass,
    Boulder,
    OakTree,
    BirdNest,
}

pub enum TileMethods {
    Dirt(TileDirt),
    Grass(TileGrass),
    Boulder(TileBoulder),
    OakTree(TileOakTree),
    BirdNest(TileBirdNest),
}

impl TileMethods {
    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        match self {
            TileMethods::Dirt(state) => state.update(time_step),
            TileMethods::Grass(state) => state.update(time_step),
            TileMethods::Boulder(state) => state.update(time_step),
            TileMethods::OakTree(state) => state.update(time_step),
            TileMethods::BirdNest(state) => state.update(time_step),
        }
    }

    pub fn render(
        &self,
        rot_time: f64,
        placement: TilePlacementType,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        match (self, placement) {
            (TileMethods::Dirt(state), TilePlacementType::World { grid_pos }) => {
                state.render(rot_time, &grid_pos, shader_color, render_pack, assets)
            }
            (TileMethods::Grass(state), TilePlacementType::World { grid_pos }) => {
                state.render(rot_time, &grid_pos, shader_color, render_pack, assets)
            }
            (TileMethods::Boulder(state), TilePlacementType::World { grid_pos }) => {
                state.render(rot_time, &grid_pos, shader_color, render_pack, assets)
            }
            (TileMethods::OakTree(state), TilePlacementType::World { grid_pos }) => {
                state.render(rot_time, &grid_pos, shader_color, render_pack, assets)
            }
            (TileMethods::BirdNest(state), TilePlacementType::World { grid_pos }) => {
                state.render(rot_time, &grid_pos, shader_color, render_pack, assets)
            }
            _ => {
                panic!("Invalid TileType, PlacementType combination.")
            }
        }
    }

    pub fn can_harvest(&self) -> bool {
        match self {
            TileMethods::Dirt(state) => state.can_harvest(),
            TileMethods::Grass(state) => state.can_harvest(),
            TileMethods::Boulder(state) => state.can_harvest(),
            TileMethods::OakTree(state) => state.can_harvest(),
            TileMethods::BirdNest(state) => state.can_harvest(),
        }
    }

    pub fn harvest(&mut self, grid_pos: GridPos) -> Vec<UpdateSignal> {
        match self {
            TileMethods::Dirt(state) => state.harvest(grid_pos),
            TileMethods::Grass(state) => state.harvest(grid_pos),
            TileMethods::Boulder(state) => state.harvest(grid_pos),
            TileMethods::OakTree(state) => state.harvest(grid_pos),
            TileMethods::BirdNest(state) => state.harvest(grid_pos),
        }
    }

    pub fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack) {
        match self {
            TileMethods::Dirt(state) => state.render_hover_info(shader_color, render_pack),
            TileMethods::Grass(state) => state.render_hover_info(shader_color, render_pack),
            TileMethods::Boulder(state) => state.render_hover_info(shader_color, render_pack),
            TileMethods::OakTree(state) => state.render_hover_info(shader_color, render_pack),
            TileMethods::BirdNest(state) => state.render_hover_info(shader_color, render_pack),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TilePlacementType {
    // Placed in the world
    World { grid_pos: GridPos },

    // Attaches to another tile
    Attachment { owner: usize },
}

pub struct TileInstance {
    pub tile_type: TileType,
    pub methods: TileMethods,
    pub placement_type: TilePlacementType,
}

impl TileType {
    pub fn can_place_here(&self, origin: GridPos, world: &World) -> bool {
        let footprint = self.get_tile_footprint();

        for p in footprint {
            let pos = origin + p;

            // check adjacency
            if !world.entity_map.contains_key(&pos) {
                if !world.valids.contains_key(&pos) {
                    return false;
                }
            }

            // check types
            match self {
                TileType::Dirt => {}
                TileType::BirdNest => {
                    let entities: Vec<usize> = world.get_entities(pos).unwrap_or(vec![]);
                    for idx in entities {
                        let tile = &world.entities[idx];

                        if tile.tile_type == TileType::OakTree {
                            return true;
                        }
                    }

                    return false;
                }
                TileType::Grass | TileType::Boulder | TileType::OakTree => {
                    if !world.entity_map.contains_key(&pos) {
                        return false;
                    }

                    let entities: Vec<usize> = world.get_entities(pos).unwrap_or(vec![]);
                    for idx in entities {
                        let tile = &world.entities[idx];

                        if tile.tile_type != TileType::Dirt {
                            return false;
                        }
                    }
                }
            };
        }

        return true;
    }

    pub fn create_instance(&self, placement_type: TilePlacementType) -> TileInstance {
        match self {
            TileType::Dirt => TileDirt::new(placement_type),
            TileType::Grass => TileGrass::new(placement_type),
            TileType::Boulder => TileBoulder::new(placement_type),
            TileType::OakTree => TileOakTree::new(placement_type),
            TileType::BirdNest => TileBirdNest::new(placement_type),
        }
    }

    // this could b emoved to the TilePlacement enum maybe
    pub fn get_tile_footprint(&self) -> Vec<GridPos> {
        match self {
            TileType::Dirt | TileType::Grass | TileType::Boulder | TileType::BirdNest => {
                vec![GridPos::new(0, 0)]
            }
            TileType::OakTree => vec![
                GridPos::new(0, 0),
                GridPos::new(1, 1),
                GridPos::new(0, 1),
                GridPos::new(1, 0),
            ],
        }
    }
}

pub fn draw_tile(
    tile_type: TileType,
    rotation: f64,
    pos: &GridPos,
    shader_color: Shader,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    let mut r = Rect::new_square(GRID_SIZE);

    r.set_center(grid_to_world(pos));

    let mut mat = Material::new();
    mat.shader = Some(shader_color);

    mat.uniforms.insert(
        "tex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: assets.get_tile_icon(&tile_type),
            texture_slot: 0,
        }),
    );

    mat.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(COLOR_WHITE.into()),
    );

    render_pack
        .commands
        .push(RenderCommand::new_rect(&r, -1.0, rotation, &mat));
}
