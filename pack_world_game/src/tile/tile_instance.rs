use crate::{
    assets::*,
    drop_table::*,
    error::Error,
    grid::*,
    save_file::{load, *},
    tile::{harvest_timer::*, tile_component::*, TileMethods, TileType},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    platform_api::*,
    render::{render_pack::*, shader::*},
};

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub grid_pos: GridPos,
    pub methods: TileMethods,

    pub components: Vec<TileComponent>,

    // for giving offset drops
    pub drop_timer: f64,
    pub drops_queue: Vec<Drop>,
    pub destroy_after_drops: bool,
}

impl TileInstance {
    pub fn new(tile_type: TileType, grid_pos: GridPos, methods: TileMethods) -> Self {
        Self {
            tile_type,
            grid_pos,
            methods,

            components: vec![],

            drop_timer: 0.0,
            drops_queue: vec![],
            destroy_after_drops: false,
        }
    }

    pub fn harvest(&mut self, world_snapshot: &WorldSnapshot, platform_api: &PlatformApi) {
        if let Some(timer) = self.get_component_harvestable_mut() {
            let drop = timer.harvest(platform_api);

            self.drops_queue.append(&mut drop.to_individual());

            match self.tile_type {
                TileType::Reed => {
                    self.destroy_after_drops = true;
                }
                _ => {}
            }
        }
    }

    pub fn can_harvest(&self) -> bool {
        if let Some(timer) = self.get_component_harvestable() {
            return timer.can_harvest();
        }

        return false;
    }

    pub fn sim_update(&mut self, delta_time: f64) -> Vec<UpdateSignal> {
        if let Some(timer) = self.get_component_harvestable_mut() {
            timer.inc(delta_time);
        }

        vec![]
    }

    pub fn update(&mut self, delta_time: f64) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        if self.drops_queue.len() > 0 {
            self.drop_timer += delta_time;

            if self.drop_timer > 0.06 {
                self.drop_timer = 0.0;

                return vec![UpdateSignal::AddHarvestDrop {
                    drop: self.drops_queue.pop().unwrap(),
                    origin: grid_to_world(&self.grid_pos),
                }];
            }
        } else {
            if self.destroy_after_drops {
                ret.push(UpdateSignal::DestroyTile {
                    pos: self.grid_pos,
                    layer: self.tile_type.get_definition().world_layer,
                });
            }
        }

        ret
    }

    pub fn get_component_harvestable(&self) -> Option<&HarvestTimer> {
        for c in &self.components {
            match c {
                TileComponent::Harvestable { timer } => {
                    return Some(timer);
                }
            }
        }

        None
    }

    pub fn get_component_harvestable_mut(&mut self) -> Option<&mut HarvestTimer> {
        for c in &mut self.components {
            match c {
                TileComponent::Harvestable { timer } => {
                    return Some(timer);
                }
            }
        }

        None
    }

    pub fn render(
        &self,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        let harvestable = self.get_component_harvestable();

        match &self.methods {
            TileMethods::Dirt(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Grass(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::Boulder(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::OakTree(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::BirdNest(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Cave(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::Shrub(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::MudPit(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::TallGrass(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::Frog(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::Water(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets)
            }
            TileMethods::Newt(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::Reed(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
            TileMethods::Clam(state) => state.render(
                harvestable.unwrap(),
                rot_time,
                pos,
                shader_color,
                render_pack,
                assets,
            ),
        }
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let type_key = format!("{}.type", key_parent);
        let grid_x_key = format!("{}.x", key_parent);
        let grid_y_key = format!("{}.y", key_parent);

        save_file.save_i32(&type_key, self.tile_type.to_index());
        save_file.save_i32(&grid_x_key, self.grid_pos.x);
        save_file.save_i32(&grid_y_key, self.grid_pos.y);

        let methods_key = format!("{}.m", key_parent);
        self.methods.save_file_write(methods_key, save_file)?;

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let type_key = format!("{}.type", key_parent);
        let grid_x_key = format!("{}.x", key_parent);
        let grid_y_key = format!("{}.y", key_parent);

        let type_index = save_file.load_i32(&type_key).unwrap();

        let tile_type: TileType = TileType::from_index(type_index)?;

        let mut grid_pos = GridPos::new(0, 0);
        grid_pos.x = save_file.load_i32(&grid_x_key).unwrap();
        grid_pos.y = save_file.load_i32(&grid_y_key).unwrap();

        let methods =
            TileMethods::save_file_load(format!("{}.m", key_parent), grid_pos, save_file)?;

        Ok(TileInstance::new(tile_type, grid_pos, methods))
    }
}
