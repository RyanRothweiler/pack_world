use crate::{
    drop_table::*,
    grid::*,
    save_file::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, *},
};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Bird Nest",
    description: "Must be placed in a tree. Adds acorn drops to adjacent grass.",
    world_layer: WorldLayer::TreeAttachment,
    footprint: vec![GridPos::new(0, 0)],

    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::OakTree {
        has_nest: false,
    })],

    new_instance: new_instance,
});

#[derive(Debug, Clone)]
pub struct TileBirdNest {
    pub tree_origin: GridPos,
}

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(
        TileType::BirdNest,
        grid_pos,
        TileMethods::BirdNest(TileBirdNest {
            tree_origin: GridPos::new(0, 0),
        }),
    );

    inst
}

impl TileBirdNest {
    pub fn tile_placed(&mut self, current_tiles: Vec<&TileInstance>) {
        for inst in current_tiles {
            if inst.tile_type == TileType::OakTree {
                self.tree_origin = inst.grid_pos;
                return;
            }
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
        let mut world_pos = grid_to_world(&self.tree_origin);
        world_pos.z += GRID_SIZE;
        world_pos.x += GRID_SIZE * 0.5;
        world_pos.y = 3.5;

        draw_tile_world_pos(
            TileType::BirdNest,
            0.0,
            &world_pos,
            true,
            render_pack,
            assets,
        );
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let x_key = format!("{}.x", key_parent);
        let y_key = format!("{}.y", key_parent);

        save_file.save_i32(&x_key, self.tree_origin.x);
        save_file.save_i32(&y_key, self.tree_origin.y);

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<TileMethods, Error> {
        let x_key = format!("{}.x", key_parent);
        let y_key = format!("{}.y", key_parent);

        let tm = TileMethods::BirdNest(TileBirdNest {
            tree_origin: GridPos::new(save_file.load_i32(&x_key)?, save_file.load_i32(&y_key)?),
        });

        Ok(tm)
    }
}

mod test {
    use super::*;
    use crate::save_file::*;

    #[test]
    fn save_load() {
        let mut save_file = SaveFile::new();

        let orig = TileBirdNest {
            tree_origin: GridPos::new(10, 20),
        };

        orig.save_file_write("nest".into(), &mut save_file).unwrap();

        match TileBirdNest::save_file_load("nest".into(), &save_file).unwrap() {
            TileMethods::BirdNest(state) => {
                assert_eq!(state.tree_origin.x, 10);
                assert_eq!(state.tree_origin.y, 20);
            }
            _ => panic!("Incorrect"),
        }
    }
}
