use crate::{
    drop_table::*,
    error::*,
    grid::*,
    save_file::load,
    state::{assets::*, *},
    tile::{tile_instance::*, tile_type::*, tiles::*},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    platform_api::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    vectors::*,
};
use std::io::{Read, Write};

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

    pub fn harvest(
        &mut self,
        grid_pos: GridPos,
        world_snapshot: &WorldSnapshot,
        platform_api: &PlatformApi,
    ) -> Option<Drop> {
        match self {
            TileMethods::Grass(state) => {
                Some(state.harvest(grid_pos, world_snapshot, platform_api))
            }
            TileMethods::Boulder(state) => Some(state.harvest(grid_pos, platform_api)),
            TileMethods::OakTree(state) => Some(state.harvest(grid_pos, platform_api)),
            TileMethods::Cave(state) => Some(state.harvest(grid_pos, platform_api)),
            TileMethods::Shrub(state) => Some(state.harvest(grid_pos, platform_api)),

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

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            TileMethods::Dirt(state) => {
                let id: i32 = 1;

                writer.write(&id.to_le_bytes())?;
            }
            TileMethods::Grass(state) => {
                let id: i32 = 2;

                writer.write(&id.to_le_bytes())?;
            }
            _ => {
                todo!("unimplmented tile write ");
            }
        }
        Ok(())
    }

    pub fn read<W: Read>(reader: &mut W) -> Result<Self, Error> {
        let id = load::read_i32(reader)?;

        match id {
            1 => Ok(TileDirt::new_methods()),
            2 => Ok(TileGrass::new_methods()),
            _ => {
                return Err(Error::UnknownTileMethodID(id));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tile::tiles::*;
    use std::io::Cursor;

    #[test]
    fn save_load_dirt() {
        let original = TileDirt::new_methods();

        let mut data: Vec<u8> = vec![];
        let mut cursor = Cursor::new(data);

        // write into buffer
        original.write(&mut cursor).unwrap();

        let save_file: Vec<u8> = cursor.get_ref().to_vec();

        // load from buffer
        let loaded: TileMethods = TileMethods::read(&mut Cursor::new(save_file)).unwrap();

        match (original, loaded) {
            (TileMethods::Dirt(x), TileMethods::Dirt(y)) => {}
            _ => panic!("Methods don't match."),
        }
    }
}
