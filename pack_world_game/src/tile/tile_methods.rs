use crate::{
    drop_table::*,
    error::*,
    grid::*,
    save_file::{load, *},
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
    MudPit(TileMudPit),
    TallGrass(TileTallGrass),
    Frog(TileFrog),
    Water(TileWater),
    Newt(TileNewt),
    Reed(TileReed),
    Clam(TileClam),
}

impl TileMethods {
    pub fn update(
        &mut self,
        origin: GridPos,
        time_step: f64,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        match self {
            TileMethods::Dirt(state) => state.update(time_step),
            TileMethods::Water(state) => state.update(time_step),
            TileMethods::Grass(state) => state.update(time_step),
            TileMethods::Boulder(state) => state.update(time_step),
            TileMethods::OakTree(state) => state.update(time_step),
            TileMethods::BirdNest(state) => state.update(time_step),
            TileMethods::Cave(state) => state.update(time_step),
            TileMethods::Shrub(state) => state.update(time_step),
            TileMethods::MudPit(state) => state.update(time_step),
            TileMethods::TallGrass(state) => state.update(time_step),
            TileMethods::Reed(state) => state.update(time_step),
            TileMethods::Frog(state) => state.update(origin, time_step, platform_api),
            TileMethods::Newt(state) => state.update(origin, time_step, platform_api),
            TileMethods::Clam(state) => state.update(time_step),
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
            TileMethods::MudPit(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::TallGrass(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Frog(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Water(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Newt(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Reed(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Clam(state) => {
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
            TileMethods::MudPit(state) => state.can_harvest(),
            TileMethods::TallGrass(state) => state.can_harvest(),
            TileMethods::Frog(state) => state.can_harvest(),
            TileMethods::Water(state) => state.can_harvest(),
            TileMethods::Newt(state) => state.can_harvest(),
            TileMethods::Reed(state) => state.can_harvest(),
            TileMethods::Clam(state) => state.can_harvest(),
        }
    }

    pub fn harvest(
        &mut self,
        grid_pos: GridPos,
        world_snapshot: &WorldSnapshot,
        platform_api: &PlatformApi,
    ) -> (Option<Drop>, Vec<UpdateSignal>) {
        match self {
            TileMethods::Grass(state) => (
                Some(state.harvest(grid_pos, world_snapshot, platform_api)),
                vec![],
            ),
            TileMethods::Boulder(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::OakTree(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::Cave(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::Shrub(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::MudPit(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::TallGrass(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::Frog(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::Newt(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::Reed(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),
            TileMethods::Clam(state) => (Some(state.harvest(grid_pos, platform_api)), vec![]),

            // these ones don't harvest
            TileMethods::Dirt(state) => (None, vec![]),
            TileMethods::Water(state) => (None, vec![]),
            TileMethods::BirdNest(state) => (None, vec![]),
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
            TileMethods::Water(state) => state.render_hover_info(shader_color, render_pack),
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
            TileMethods::MudPit(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::TallGrass(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::Frog(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::Newt(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::Reed(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
            TileMethods::Clam(state) => {
                state.render_hover_info(y_offset, shader_color, render_pack)
            }
        }
    }

    /// Convert the tile into a tilesnapshot
    pub fn into_snapshot(&self) -> TileSnapshot {
        match self {
            TileMethods::Dirt(state) => TileSnapshot::Dirt,
            TileMethods::Water(state) => TileSnapshot::Water,
            TileMethods::Grass(state) => TileSnapshot::Grass,
            TileMethods::Boulder(state) => TileSnapshot::Boulder,
            TileMethods::OakTree(state) => TileSnapshot::OakTree {
                has_nest: state.has_nest,
            },
            TileMethods::BirdNest(state) => TileSnapshot::BirdNest,
            TileMethods::Cave(state) => TileSnapshot::Cave,
            TileMethods::Shrub(state) => TileSnapshot::Shrub,
            TileMethods::MudPit(state) => TileSnapshot::MudPit,
            TileMethods::TallGrass(state) => TileSnapshot::TallGrass,
            TileMethods::Frog(state) => TileSnapshot::Frog,
            TileMethods::Newt(state) => TileSnapshot::Newt,
            TileMethods::Reed(state) => TileSnapshot::Reed,
            TileMethods::Clam(state) => TileSnapshot::Clam,
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

    pub fn update_world_conditions(&mut self, grid_pos: GridPos, snapshot: &WorldSnapshot) {
        match self {
            TileMethods::Grass(state) => state.update_world_conditions(grid_pos, snapshot),
            _ => {}
        }
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let type_key = format!("{}.t", key_parent);
        let state_key = format!("{}.s", key_parent);

        match self {
            TileMethods::Dirt(state) => {
                let id: i32 = 1;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Grass(state) => {
                let id: i32 = 2;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Boulder(state) => {
                let id: i32 = 3;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::OakTree(state) => {
                let id: i32 = 4;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::BirdNest(state) => {
                let id: i32 = 5;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Cave(state) => {
                let id: i32 = 6;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Shrub(state) => {
                let id: i32 = 7;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::MudPit(state) => {
                let id: i32 = 8;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::TallGrass(state) => {
                let id: i32 = 9;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Frog(state) => {
                let id: i32 = 10;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Water(state) => {
                let id: i32 = 11;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Newt(state) => {
                let id: i32 = 12;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Reed(state) => {
                let id: i32 = 13;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Clam(state) => {
                let id: i32 = 14;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
        }

        Ok(())
    }

    pub fn save_file_load(
        key_parent: String,
        grid_pos: GridPos,
        save_file: &SaveFile,
    ) -> Result<Self, Error> {
        let type_key = format!("{}.t", key_parent);
        let state_key = format!("{}.s", key_parent);

        let id = save_file.load_i32(&type_key).unwrap();
        match id {
            1 => Ok(TileDirt::new_methods(GridPos::new(0, 0))),
            2 => Ok(TileGrass::save_file_load(state_key, save_file)?),
            3 => Ok(TileBoulder::save_file_load(state_key, save_file)?),
            4 => Ok(TileOakTree::save_file_load(state_key, save_file)?),
            5 => Ok(TileBirdNest::save_file_load(state_key, save_file)?),
            6 => Ok(TileCave::save_file_load(state_key, save_file)?),
            7 => Ok(TileShrub::save_file_load(state_key, save_file)?),
            8 => Ok(TileMudPit::save_file_load(state_key, save_file)?),
            9 => Ok(TileTallGrass::save_file_load(state_key, save_file)?),
            10 => Ok(TileFrog::save_file_load(state_key, grid_pos, save_file)?),
            11 => Ok(TileWater::new_methods(GridPos::new(0, 0))),
            12 => Ok(TileNewt::save_file_load(state_key, grid_pos, save_file)?),
            13 => Ok(TileReed::save_file_load(state_key, save_file)?),
            14 => Ok(TileClam::save_file_load(state_key, save_file)?),
            _ => {
                return Err(Error::UnknownTileMethodID(id));
            }
        }
    }
}

mod tests {
    use super::*;
    use crate::save_file::*;

    #[test]
    fn save_load() {
        let mut save_file = SaveFile::new();

        TileDirt::new_methods(GridPos::new(0, 0))
            .save_file_write("dirt".into(), &mut save_file)
            .unwrap();
        TileGrass::new_methods(GridPos::new(0, 0))
            .save_file_write("grass".into(), &mut save_file)
            .unwrap();
        TileBoulder::new_methods(GridPos::new(0, 0))
            .save_file_write("boulder".into(), &mut save_file)
            .unwrap();
        TileOakTree::new_methods(GridPos::new(0, 0))
            .save_file_write("oak tree".into(), &mut save_file)
            .unwrap();
        TileBirdNest::new_methods(GridPos::new(0, 0))
            .save_file_write("bird nest".into(), &mut save_file)
            .unwrap();
        TileCave::new_methods(GridPos::new(0, 0))
            .save_file_write("cave".into(), &mut save_file)
            .unwrap();
        TileShrub::new_methods(GridPos::new(0, 0))
            .save_file_write("shrub".into(), &mut save_file)
            .unwrap();
        TileMudPit::new_methods(GridPos::new(0, 0))
            .save_file_write("mudpit".into(), &mut save_file)
            .unwrap();
        TileTallGrass::new_methods(GridPos::new(0, 0))
            .save_file_write("tall_grass".into(), &mut save_file)
            .unwrap();
        TileFrog::new_methods(GridPos::new(5, 5))
            .save_file_write("frog".into(), &mut save_file)
            .unwrap();
        TileWater::new_methods(GridPos::new(0, 0))
            .save_file_write("water".into(), &mut save_file)
            .unwrap();
        TileNewt::new_methods(GridPos::new(5, 5))
            .save_file_write("newt".into(), &mut save_file)
            .unwrap();
        TileReed::new_methods(GridPos::new(0, 0))
            .save_file_write("reed".into(), &mut save_file)
            .unwrap();
        TileClam::new_methods(GridPos::new(0, 0))
            .save_file_write("clam".into(), &mut save_file)
            .unwrap();

        match TileMethods::save_file_load("dirt".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Dirt(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("grass".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Grass(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("boulder".into(), GridPos::new(0, 0), &save_file).unwrap()
        {
            TileMethods::Boulder(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("oak tree".into(), GridPos::new(0, 0), &save_file)
            .unwrap()
        {
            TileMethods::OakTree(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("bird nest".into(), GridPos::new(0, 0), &save_file)
            .unwrap()
        {
            TileMethods::BirdNest(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("cave".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Cave(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("shrub".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Shrub(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("mudpit".into(), GridPos::new(0, 0), &save_file).unwrap()
        {
            TileMethods::MudPit(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("tall_grass".into(), GridPos::new(0, 0), &save_file)
            .unwrap()
        {
            TileMethods::TallGrass(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("frog".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Frog(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("water".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Water(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("newt".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Newt(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("reed".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Reed(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("clam".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Clam(state) => {}
            _ => panic!("Incorrect"),
        }
    }
}
