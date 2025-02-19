#![allow(dead_code)]

use crate::{
    drop_table::*,
    grid::*,
    item::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, *},
    world::*,
};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
};

pub const TITLE: &str = "Grass";

const HARVEST_SECONDS: f64 = 20.0;

#[derive(Debug)]
pub struct TileGrass {
    pub harvest_timer: HarvestTimer,
}

impl TileGrass {
    pub fn new(grid_pos: GridPos) -> TileInstance {
        TileInstance {
            grid_pos,
            tile_type: TileType::Grass,
            methods: TileMethods::Grass(TileGrass {
                harvest_timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Grass),
            }),
        }
    }

    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        self.harvest_timer.inc(time_step);
        vec![]
    }

    pub fn can_harvest(&self) -> bool {
        self.harvest_timer.can_harvest()
    }

    pub fn harvest(
        &mut self,
        grid_pos: GridPos,
        world_snapshot: &WorldSnapshot,
    ) -> Vec<UpdateSignal> {
        let mut nest_adj = false;

        for adj_pos in grid_pos.to_adjacents_iter() {
            for t in world_snapshot.get_pos_snapshot(adj_pos) {
                match t {
                    TileSnapshot::OakTree { has_nest } => {
                        if has_nest {
                            nest_adj = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut drop_table = self.harvest_timer.table.clone();
        if nest_adj {
            drop_table = drop_table.add_entry((EntryOutput::new_item(ItemType::Acorn, 1), 2.0));
        }

        self.harvest_timer.reset();
        vec![UpdateSignal::AddHarvestDrop {
            drop: drop_table.get_drop(),
            origin: grid_to_world(&grid_pos),
        }]
    }

    pub fn render_hover_info(
        &self,
        y_offset: f64,
        shader_color: Shader,
        render_pack: &mut RenderPack,
    ) {
        let base: VecTwo = VecTwo::new(450.0, 110.0 + y_offset);
        let mut r = Rect::new_top_size(base, 200.0, 10.0);

        draw_progress_bar(
            self.harvest_timer.percent_done(),
            &r,
            shader_color,
            render_pack,
        );
    }

    pub fn render(
        &self,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);

        let mut rotation: f64 = 0.0;
        if self.can_harvest() {
            rotation = f64::sin(rot_time) * 7.0;
        }

        draw_tile(
            TileType::Grass,
            rotation,
            pos,
            shader_color,
            render_pack,
            assets,
        );
    }
}
