use crate::{
    drop_table::*,
    grid::*,
    save_file::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, *},
};
use gengar_engine::{
    color::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Oak Tree",
    description: "Drops construction resources.",
    world_layer: WorldLayer::Floor,
    footprint: GridPos::new(0, 0).to_rect_iter(2, 2).collect(),

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    build_methods: TileOakTree::new_methods,
    add_components: TileOakTree::add_components,
});

const HARVEST_SECONDS: f64 = 360.0;

#[derive(Debug)]
pub struct TileOakTree {
    // TODO remove has_nest and just use the nest_id option
    pub has_nest: bool,
    pub nest_id: Option<EntityID>,
}

impl TileOakTree {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        TileMethods::OakTree(TileOakTree {
            has_nest: false,
            nest_id: None,
        })
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {
        inst.components.push(TileComponent::Harvestable {
            timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::OakTree),
        });
    }

    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        vec![]
    }

    pub fn tile_placed_ontop(&mut self, tile_type: TileType, top_id: EntityID) {
        if tile_type == TileType::BirdNest {
            self.has_nest = true;
            self.nest_id = Some(top_id);
        }
    }

    pub fn render(
        &self,
        time_comp: &HarvestTimer,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);

        let mut rotation: f64 = 0.0;
        if time_comp.can_harvest() {
            rotation = f64::sin(rot_time) * 7.0;
        }

        // render tree
        {
            let mut r = Rect::new_square(GRID_SIZE * 2.0);
            let pos_world = grid_to_world(pos) + VecTwo::new(GRID_SIZE * 0.5, GRID_SIZE * 0.5);
            r.set_center(pos_world);

            let mut mat = Material::new();
            mat.shader = Some(shader_color);

            mat.uniforms.insert(
                "tex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: assets.get_tile_icon(&TileType::OakTree),
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
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let timer_key = format!("{}.h", key_parent);
        let has_nest_key = format!("{}.hn", key_parent);
        let nest_entity_id = format!("{}.hne", key_parent);

        // self.harvest_timer.save_file_write(timer_key, save_file)?;

        save_file.save_bool(&has_nest_key, self.has_nest);
        if self.has_nest {
            save_file.save_u64(&nest_entity_id, self.nest_id.unwrap().id);
        }

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<TileMethods, Error> {
        let timer_key = format!("{}.h", key_parent);
        let has_nest_key = format!("{}.hn", key_parent);
        let nest_entity_id = format!("{}.hne", key_parent);

        // let harvest_timer = HarvestTimer::save_file_load(timer_key, save_file)?;
        let has_nest: bool = save_file.load_bool(&has_nest_key).unwrap();
        let mut nest_entity: Option<EntityID> = None;
        if has_nest {
            let eid = save_file.load_u64(&nest_entity_id).unwrap();
            nest_entity = Some(EntityID { id: eid });
        }

        let tm = TileMethods::OakTree(TileOakTree {
            has_nest: has_nest,
            nest_id: nest_entity,
            // harvest_timer: harvest_timer,
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

        let orig = TileOakTree {
            has_nest: true,
            nest_id: Some(EntityID { id: 100 }),
            harvest_timer: HarvestTimer::new(0.0, FixedTableID::Boulder),
        };

        orig.save_file_write("tree".into(), &mut save_file).unwrap();

        match TileOakTree::save_file_load("tree".into(), &save_file).unwrap() {
            TileMethods::OakTree(state) => {
                assert_eq!(state.has_nest, true);
                assert_eq!(state.nest_id, Some(EntityID { id: 100 }));
            }
            _ => panic!("Incorrect"),
        }
    }
}
