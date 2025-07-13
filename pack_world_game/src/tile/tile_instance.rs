use crate::{
    assets::*,
    drop_table::*,
    error::Error,
    grid::*,
    save_file::{load, *},
    tile::*,
    tile::{harvest_timer::*, tile_component::*, TileMethods, TileType},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    platform_api::*,
    render::{render_pack::*, shader::*},
    ui::*,
};

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub grid_pos: GridPos,

    pub components: Vec<TileComponent>,

    // for giving offset drops
    pub drop_timer: f64,
    pub drops_queue: Vec<Drop>,
    pub destroy_after_drops: bool,

    methods: TileMethods,
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
            _ => {}
        }
    }

    pub fn render_hover_info(
        &self,
        harvestable: Option<&HarvestTimer>,
        y_offset: f64,
        shader_color: Shader,
        render_pack: &mut RenderPack,
    ) {
        let base: VecTwo = VecTwo::new(450.0, 110.0 + y_offset);
        let r = Rect::new_top_size(base, 200.0, 10.0);

        if let Some(time_comp) = harvestable {
            draw_progress_bar(time_comp.percent_done(), &r, shader_color, render_pack);
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

    pub fn sim_update(&mut self, delta_time: f64, platform_api: &PlatformApi) -> Vec<UpdateSignal> {
        if let Some(timer) = self.get_component_harvestable_mut() {
            let drop_opt = timer.inc(delta_time, platform_api);
            if let Some(drop) = drop_opt {
                self.drops_queue.append(&mut drop.to_individual());
            }
        }

        vec![]
    }

    pub fn update_world_conditions(&mut self, world_snapshot: &WorldSnapshot) {
        let gp = self.grid_pos;
        if let Some(timer) = self.get_component_harvestable_mut() {
            timer.update_world_conditions(gp, world_snapshot);
        }
    }

    pub fn update(&mut self, delta_time: f64, platform_api: &PlatformApi) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];
        let grid_pos: GridPos = self.grid_pos;

        // Wandering behavior
        if let Some(wander_state) = self.get_component_wander_mut() {
            wander_state.update(grid_pos, delta_time, platform_api);
        }

        // Update drop displays
        {
            if self.drops_queue.len() > 0 {
                self.drop_timer += delta_time;

                if self.drop_timer > 0.06 {
                    self.drop_timer = 0.0;

                    return vec![UpdateSignal::AddHarvestDrop {
                        drop: self.drops_queue.pop().unwrap(),
                        origin: self.grid_pos,
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
        }

        ret
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
                let harvestable = self.get_component_harvestable();
                let wander = self.get_component_wander();

                // harvesting rotation
                let mut rotation: f64 = 0.0;

                if let Some(time_comp) = harvestable {
                    if time_comp.can_harvest() {
                        rotation = f64::sin(rot_time) * 7.0;
                    }
                }

                // wander position
                let mut render_pos = grid_to_world(pos);
                if let Some(wander_state) = wander {
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
        }
    }

    pub fn get_component_harvestable(&self) -> Option<&HarvestTimer> {
        for c in &self.components {
            match c {
                TileComponent::Harvestable { timer } => {
                    return Some(timer);
                }
                _ => {}
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
                _ => {}
            }
        }

        None
    }

    pub fn get_component_wander(&self) -> Option<&WanderState> {
        for c in &self.components {
            match c {
                TileComponent::Wander { state } => {
                    return Some(state);
                }
                _ => {}
            }
        }

        None
    }

    pub fn get_component_wander_mut(&mut self) -> Option<&mut WanderState> {
        for c in &mut self.components {
            match c {
                TileComponent::Wander { state } => {
                    return Some(state);
                }
                _ => {}
            }
        }

        None
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

        // Save compnents
        // This assumes a tile doesn't have multiple of the same type of component
        for c in &self.components {
            c.save_file_write(comp_key.clone(), save_file)?;
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
        for c in &mut inst.components {
            c.save_file_load(comp_key.clone(), save_file)?;
        }

        Ok(inst)
    }
}
