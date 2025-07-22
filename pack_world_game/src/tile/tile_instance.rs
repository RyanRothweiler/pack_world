use crate::{
    assets::*,
    constants::*,
    drop_table::*,
    error::Error,
    grid::*,
    save_file::{load, *},
    tile::*,
    tile::{TileMethods, TileType},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    platform_api::*,
    render::{render_pack::*, shader::*},
    time::*,
    ui::*,
};

pub mod tile_comp_auto_death;
pub mod tile_comp_harvest;
pub mod tile_comp_harvest_others;
pub mod tile_comp_wander;

pub use tile_comp_auto_death::*;
pub use tile_comp_harvest::*;
pub use tile_comp_harvest_others::*;
pub use tile_comp_wander::*;

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub grid_pos: GridPos,

    // for giving offset drops
    pub drop_timer: f64,
    pub drops_queue: Vec<Drop>,
    pub destroy_after_drops: bool,

    methods: TileMethods,

    // component like things.
    pub comp_wander: Option<TileCompWander>,
    pub comp_harvest: Option<TileCompHarvest>,
    pub comp_auto_death: Option<TileCompAutoDeath>,
    pub comp_harvest_others: Option<TileCompHarvestOthers>,
}

impl TileInstance {
    pub fn new(tile_type: TileType, grid_pos: GridPos, methods: TileMethods) -> Self {
        Self {
            tile_type,
            grid_pos,
            methods,

            drop_timer: 0.0,
            drops_queue: vec![],
            destroy_after_drops: false,

            comp_wander: None,
            comp_harvest: None,
            comp_auto_death: None,
            comp_harvest_others: None,
        }
    }

    /// Some other tile is placed ontop of this one.
    /// top_id is the entity_id of the newly placed tile.
    pub fn tile_placed_ontop(&mut self, tile_type: TileType, top_id: EntityID) {
        match &mut self.methods {
            TileMethods::OakTree(state) => state.tile_placed_ontop(tile_type, top_id),

            // Default is that tile doesn't care
            _ => {}
        }
    }

    pub fn tile_placed(&mut self, current_tiles: Vec<&TileInstance>) {
        match &mut self.methods {
            TileMethods::BirdNest(state) => state.tile_placed(current_tiles),

            // Default is that the tile doesn't care
            _ => {}
        }
    }

    pub fn render_hover_info(
        &self,
        y_offset: f64,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        ui_context: &mut UIContext,
        mut ui_frame_state: &mut UIFrameState,
    ) {
        let base: VecTwo = VecTwo::new(450.0, 110.0 + y_offset);
        let r = Rect::new_top_size(base, 200.0, 10.0);

        if let Some(time_comp) = &self.comp_harvest {
            draw_progress_bar(time_comp.percent_done(), &r, shader_color, render_pack);

            let disp = Time::new(TimeUnit::Seconds(time_comp.length() - time_comp.time)).display();
            draw_text(
                &disp,
                base + VecTwo::new(210.0, 10.0),
                *THEME_TEXT_MUT,
                &ui_context.font_body.clone(),
                &mut ui_frame_state,
                ui_context,
            );
        }

        if let Some(ho) = &self.comp_harvest_others {
            draw_progress_bar(ho.perc_done(), &r, shader_color, render_pack);
        }
    }

    pub fn harvest(&mut self, world_snapshot: &WorldSnapshot, platform_api: &PlatformApi) {
        if let Some(timer) = &mut self.comp_harvest {
            if timer.can_harvest() {
                let drop = timer.harvest(world_snapshot, &self.grid_pos, platform_api);

                self.drops_queue.append(&mut drop.to_individual());

                self.destroy_after_drops = timer.destroy_after_harvest;
            }
        }
    }

    pub fn can_harvest(&self) -> bool {
        if let Some(timer) = &self.comp_harvest {
            return timer.can_harvest();
        }

        return false;
    }

    /// World simulation update
    pub fn sim_update(
        &mut self,
        delta_time: f64,
        world_snapshot: &WorldSnapshot,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        let mut sigs: Vec<UpdateSignal> = vec![];

        // Harvestable
        if let Some(timer) = &mut self.comp_harvest {
            let drop_opt = timer.inc(delta_time, world_snapshot, &self.grid_pos, platform_api);
            if let Some(drop) = drop_opt {
                self.drops_queue.append(&mut drop.to_individual());
            }
        }

        // Auto death
        if let Some(ad) = &mut self.comp_auto_death {
            ad.inc(Time::new(TimeUnit::Seconds(delta_time)));
            if !ad.alive() {
                sigs.push(self.destroy_self_sig());
            }
        }

        // Harvesting others
        if let Some(ho) = &mut self.comp_harvest_others {
            let mut us = ho.update(
                Time::new(TimeUnit::Seconds(delta_time)),
                &self.grid_pos,
                world_snapshot,
            );
            sigs.append(&mut us);
        }

        sigs
    }

    pub fn update_world_conditions(&mut self, world_snapshot: &WorldSnapshot) {
        let gp = self.grid_pos;
        if let Some(timer) = &mut self.comp_harvest {
            timer.update_world_conditions(gp, world_snapshot);
        }
    }

    /// Game frame update
    pub fn update(&mut self, delta_time: f64, platform_api: &PlatformApi) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];
        let grid_pos: GridPos = self.grid_pos;

        // Wandering behavior
        if let Some(wander_state) = &mut self.comp_wander {
            wander_state.update(grid_pos, delta_time, platform_api);
        }

        // Update drop displays
        {
            if self.drops_queue.len() > 0 {
                self.drop_timer += delta_time;

                if self.drop_timer > DROP_TIME_GUTTER_S {
                    self.drop_timer = 0.0;

                    return vec![UpdateSignal::AddHarvestDrop {
                        drop: self.drops_queue.pop().unwrap(),
                        origin: self.grid_pos,
                    }];
                }
            } else {
                if self.destroy_after_drops {
                    ret.push(self.destroy_self_sig());
                }
            }
        }

        ret
    }

    fn destroy_self_sig(&self) -> UpdateSignal {
        UpdateSignal::DestroyTile {
            pos: self.grid_pos,
            layer: self.tile_type.get_definition().world_layer,
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
        match &self.methods {
            TileMethods::BirdNest(state) => {
                state.render(rot_time, pos, shader_color, render_pack, assets);
            }
            _ => {
                // harvesting rotation
                let mut rotation: f64 = 0.0;

                // harvesting
                if let Some(time_comp) = &self.comp_harvest {
                    if time_comp.can_harvest() {
                        rotation = f64::sin(rot_time) * 7.0;
                    }
                }

                // wander position
                let mut render_pos = grid_to_world(pos);
                if let Some(wander_state) = &self.comp_wander {
                    render_pos = wander_state.curr_world_pos;
                }

                draw_tile_world_pos(
                    self.tile_type,
                    rotation,
                    &render_pos,
                    true,
                    render_pack,
                    assets,
                );
            }
        }
    }

    /// Convert the tile into a tilesnapshot
    pub fn into_snapshot(&self) -> TileSnapshot {
        // TODO make this conversion part of the tile definition

        /*
        I made multiple attempts to to remove the need for the snapshots.
        But this actually seems the best. Otherwise we need equals checks on the
        individual tile states. And also each world condition definition would need
        to include unecessary tile state data.
        Also separation of world conditions into its own structure would be needed.
        This all was more complicated and not obviously better than just
        manually doing conversions into a new structure.
        */

        match &self.methods {
            TileMethods::Dirt => TileSnapshot::Dirt,
            TileMethods::Water => TileSnapshot::Water,
            TileMethods::Grass => TileSnapshot::Grass,
            TileMethods::Boulder => TileSnapshot::Boulder,
            TileMethods::OakTree(state) => TileSnapshot::OakTree {
                has_nest: state.has_nest,
            },
            TileMethods::BirdNest(state) => TileSnapshot::BirdNest,
            TileMethods::Cave => TileSnapshot::Cave,
            TileMethods::Shrub => TileSnapshot::Shrub,
            TileMethods::MudPit => TileSnapshot::MudPit,
            TileMethods::TallGrass => TileSnapshot::TallGrass,
            TileMethods::Frog => TileSnapshot::Frog,
            TileMethods::Newt => TileSnapshot::Newt,
            TileMethods::Reed => TileSnapshot::Reed,
            TileMethods::Clam => TileSnapshot::Clam,
            TileMethods::MudFish => TileSnapshot::MudFish,
            TileMethods::Spring => TileSnapshot::Spring,
            TileMethods::Kelp => TileSnapshot::Kelp,
            TileMethods::Crab => TileSnapshot::Crab,
            TileMethods::MudHenge => TileSnapshot::MudHenge,
            TileMethods::MudChicken => TileSnapshot::MudChicken,
            TileMethods::Goblin => TileSnapshot::Goblin,
            TileMethods::MudPig => TileSnapshot::MudPig,
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
        let comp_key = format!("{}.comp", key_parent);

        save_file.save_i32(&type_key, self.tile_type.to_index());
        save_file.save_i32(&grid_x_key, self.grid_pos.x);
        save_file.save_i32(&grid_y_key, self.grid_pos.y);

        let methods_key = format!("{}.m", key_parent);
        self.methods.save_file_write(methods_key, save_file)?;

        if let Some(harvest) = &self.comp_harvest {
            let key = format!("{}.ht", comp_key);
            harvest.save_file_write(key, save_file)?;
        }

        if let Some(ad) = &self.comp_auto_death {
            let key = format!("{}.ad", comp_key);
            ad.save_file_write(key, save_file)?;
        }

        if let Some(comp) = &self.comp_harvest_others {
            let key = format!("{}.ho", comp_key);
            save_file.save_f64(&key, comp.timer.as_milliseconds().value());
        }

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let type_key = format!("{}.type", key_parent);
        let grid_x_key = format!("{}.x", key_parent);
        let grid_y_key = format!("{}.y", key_parent);
        let comp_key = format!("{}.comp", key_parent);

        let type_index = save_file.load_i32(&type_key)?;

        let tile_type: TileType = TileType::from_index(type_index)?;

        let mut grid_pos = GridPos::new(0, 0);
        grid_pos.x = save_file.load_i32(&grid_x_key)?;
        grid_pos.y = save_file.load_i32(&grid_y_key)?;

        let methods =
            TileMethods::save_file_load(format!("{}.m", key_parent), grid_pos, save_file)?;

        let mut inst = (tile_type.get_definition().new_instance)(grid_pos);

        if let Some(harvest) = &inst.comp_harvest {
            let orig_table = harvest.table;

            let key = format!("{}.ht", comp_key);
            inst.comp_harvest = Some(TileCompHarvest::save_file_load(key, save_file)?);
            inst.comp_harvest.as_mut().unwrap().table = orig_table;
        }

        if let Some(ad) = &inst.comp_auto_death {
            let key = format!("{}.ad", comp_key);
            inst.comp_auto_death = Some(TileCompAutoDeath::save_file_load(key, save_file)?);
        }

        if let Some(comp) = &mut inst.comp_harvest_others.as_mut() {
            let key = format!("{}.ho", comp_key);
            comp.timer.ms = save_file.load_f64(&key)?;
        }

        Ok(inst)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::save_file::*;

    #[test]
    fn grass_saving() {
        let inst_orig = crate::tiles::tile_cave::new_instance(GridPos::new(10, 5));

        let mut save_file = SaveFile::new();
        inst_orig
            .save_file_write("cave".into(), &mut save_file)
            .unwrap();

        let inst_loaded = TileInstance::save_file_load("cave".into(), &save_file).unwrap();

        assert_eq!(inst_orig.grid_pos, inst_loaded.grid_pos);
        assert!(inst_loaded.comp_auto_death.is_none());
        assert!(inst_loaded.comp_wander.is_none());
        assert_eq!(
            inst_orig.comp_harvest.unwrap().table,
            inst_loaded.comp_harvest.unwrap().table
        );
    }
}
