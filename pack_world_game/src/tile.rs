use crate::{
    drop_table::*,
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

use tiles::{
    tile_bird_nest::TileBirdNest, tile_boulder::TileBoulder, tile_cave::TileCave,
    tile_dirt::TileDirt, tile_grass::TileGrass, tile_oak_tree::TileOakTree, tile_shrub::TileShrub,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum TileType {
    Dirt,
    Grass,
    Boulder,
    OakTree,
    BirdNest,
    Cave,
    Shrub,
}

// TOOD create a tile definition. and one method to return that definition instead of individual methods for each field.
impl TileType {
    pub fn user_title(&self) -> &str {
        match self {
            TileType::Dirt => tiles::tile_dirt::TITLE,
            TileType::Grass => tiles::tile_grass::TITLE,
            TileType::Boulder => tiles::tile_boulder::TITLE,
            TileType::OakTree => tiles::tile_oak_tree::TITLE,
            TileType::BirdNest => tiles::tile_bird_nest::TITLE,
            TileType::Cave => tiles::tile_cave::TITLE,
            TileType::Shrub => tiles::tile_shrub::TITLE,
        }
    }

    pub fn user_description(&self) -> Option<&str> {
        match self {
            TileType::Dirt => Some(tiles::tile_dirt::DESC),
            _ => None,
        }
    }

    pub fn get_layer(&self) -> WorldLayer {
        match self {
            TileType::Dirt => WorldLayer::Ground,
            TileType::BirdNest => WorldLayer::TreeAttachment,
            TileType::Boulder
            | TileType::OakTree
            | TileType::Cave
            | TileType::Shrub
            | TileType::Grass => WorldLayer::Floor,
        }
    }

    /// Can you place the tile here
    pub fn can_place_here(&self, origin: GridPos, world: &World) -> bool {
        let footprint = self.get_tile_footprint();
        for p in footprint {
            let pos = origin + p;

            let val = match self {
                TileType::Dirt => TileDirt::can_place(pos, world),
                TileType::Grass => TileGrass::can_place(pos, world),
                TileType::Boulder => TileBoulder::can_place(pos, world),
                TileType::OakTree => TileOakTree::can_place(pos, world),
                TileType::Cave => TileCave::can_place(pos, world),
                TileType::Shrub => TileShrub::can_place(pos, world),
                TileType::BirdNest => TileBirdNest::can_place(pos, world),
            };

            if !val {
                return false;
            }
        }

        return true;
    }

    pub fn create_instance(&self, grid_pos: GridPos) -> TileInstance {
        let methods = match self {
            TileType::Dirt => TileDirt::new_methods(),
            TileType::Grass => TileGrass::new_methods(),
            TileType::Boulder => TileBoulder::new_methods(),
            TileType::OakTree => TileOakTree::new_methods(),
            TileType::BirdNest => TileBirdNest::new_methods(),
            TileType::Cave => TileCave::new_methods(),
            TileType::Shrub => TileShrub::new_methods(),
        };

        TileInstance::new(*self, grid_pos, methods)
    }

    pub fn get_tile_footprint(&self) -> Vec<GridPos> {
        match self {
            TileType::Dirt
            | TileType::Grass
            | TileType::Boulder
            | TileType::Shrub
            | TileType::BirdNest
            | TileType::Cave => {
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

/// This is just manual dynamic dispact because Dyn breaks hot realoding.
#[derive(Debug)]
pub enum TileMethods {
    Dirt(TileDirt),
    Grass(TileGrass),
    Boulder(TileBoulder),
    OakTree(TileOakTree),
    BirdNest(TileBirdNest),
    Cave(TileCave),
    Shrub(TileShrub),
}

impl TileMethods {
    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        match self {
            TileMethods::Dirt(state) => state.update(time_step),
            TileMethods::Grass(state) => state.update(time_step),
            TileMethods::Boulder(state) => state.update(time_step),
            TileMethods::OakTree(state) => state.update(time_step),
            TileMethods::BirdNest(state) => state.update(time_step),
            TileMethods::Cave(state) => state.update(time_step),
            TileMethods::Shrub(state) => state.update(time_step),
        }
    }

    pub fn render(
        &self,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        match self {
            TileMethods::Dirt(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Grass(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Boulder(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::OakTree(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::BirdNest(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Cave(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Shrub(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
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
            TileMethods::Cave(state) => state.can_harvest(),
            TileMethods::Shrub(state) => state.can_harvest(),
        }
    }

    pub fn harvest(&mut self, grid_pos: GridPos, world_snapshot: &WorldSnapshot) -> Option<Drop> {
        match self {
            TileMethods::Grass(state) => Some(state.harvest(grid_pos, world_snapshot)),
            TileMethods::Boulder(state) => Some(state.harvest(grid_pos)),
            TileMethods::OakTree(state) => Some(state.harvest(grid_pos)),
            TileMethods::Cave(state) => Some(state.harvest(grid_pos)),
            TileMethods::Shrub(state) => Some(state.harvest(grid_pos)),

            // these ones don't harvest
            TileMethods::Dirt(state) => None,
            TileMethods::BirdNest(state) => None,
        }
    }

    pub fn render_hover_info(
        &self,
        y_offset: f64,
        shader_color: Shader,
        render_pack: &mut RenderPack,
    ) {
        match self {
            TileMethods::Dirt(state) => state.render_hover_info(shader_color, render_pack),
            TileMethods::Grass(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::Boulder(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::OakTree(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::BirdNest(state) => state.render_hover_info(shader_color, render_pack),
            TileMethods::Cave(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::Shrub(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
        }
    }

    /// Convert the tile into a tilesnapshot
    pub fn into_snapshot(&self) -> TileSnapshot {
        match self {
            TileMethods::Dirt(state) => TileSnapshot::Dirt,
            TileMethods::Grass(state) => TileSnapshot::Grass,
            TileMethods::Boulder(state) => TileSnapshot::Boulder,
            TileMethods::OakTree(state) => TileSnapshot::OakTree {
                has_nest: state.has_nest,
            },
            TileMethods::BirdNest(state) => TileSnapshot::BirdNest,
            TileMethods::Cave(state) => TileSnapshot::Cave,
            TileMethods::Shrub(state) => TileSnapshot::Shrub,
        }
    }

    /// Some other tile is placed ontop of this one.
    /// top_id is the entity_id of the newly placed tile.
    pub fn tile_placed_ontop(&mut self, tile_type: TileType, top_id: EntityID) {
        match self {
            TileMethods::OakTree(state) => state.tile_placed_ontop(tile_type, top_id),

            // Default is that tile doesn't care
            _ => {}
        }
    }

    pub fn tile_placed(&mut self, current_tiles: Vec<&TileInstance>) {
        match self {
            TileMethods::BirdNest(state) => state.tile_placed(current_tiles),
            _ => {}
        }
    }
}

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub grid_pos: GridPos,
    pub methods: TileMethods,

    // for giving offset drops
    pub drop_timer: f64,
    pub drops_queue: Vec<Drop>,
}

impl TileInstance {
    pub fn new(tile_type: TileType, grid_pos: GridPos, methods: TileMethods) -> Self {
        Self {
            tile_type,
            grid_pos,
            methods,
            drop_timer: 0.0,
            drops_queue: vec![],
        }
    }

    pub fn harvest(&mut self, world_snapshot: &WorldSnapshot) {
        let mut new_drop = self.methods.harvest(self.grid_pos, world_snapshot);

        match new_drop {
            Some(drop) => {
                self.drops_queue.append(&mut drop.to_individual());
            }
            None => {
                println!("Attempted to harvest something which isn't harvestable.");
                println!(
                    "This is fine. Nothing will break. But this indicates an issue somewhere."
                );
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) -> Vec<UpdateSignal> {
        if self.drops_queue.len() > 0 {
            self.drop_timer += delta_time;

            if self.drop_timer > 0.06 {
                self.drop_timer = 0.0;

                return vec![UpdateSignal::AddHarvestDrop {
                    drop: self.drops_queue.pop().unwrap(),
                    origin: grid_to_world(&self.grid_pos),
                }];
            }
        }

        vec![]
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
