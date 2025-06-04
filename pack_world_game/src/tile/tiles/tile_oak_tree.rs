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
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = 360.0;

#[derive(Debug, Clone)]
pub struct TileOakTree {
    // TODO remove has_nest and just use the nest_id option
    pub has_nest: bool,
    pub nest_id: Option<EntityID>,
}

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(
        TileType::OakTree,
        grid_pos,
        TileMethods::OakTree(TileOakTree {
            has_nest: false,
            nest_id: None,
        }),
    );

    inst.components.push(TileComponent::Harvestable {
        timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::OakTree),
    });

    inst
}

impl TileOakTree {
    pub fn tile_placed_ontop(&mut self, tile_type: TileType, top_id: EntityID) {
        if tile_type == TileType::BirdNest {
            self.has_nest = true;
            self.nest_id = Some(top_id);
        }
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let has_nest_key = format!("{}.hn", key_parent);
        let nest_entity_id = format!("{}.hne", key_parent);

        save_file.save_bool(&has_nest_key, self.has_nest);
        if self.has_nest {
            save_file.save_u64(&nest_entity_id, self.nest_id.unwrap().id);
        }

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<TileMethods, Error> {
        let has_nest_key = format!("{}.hn", key_parent);
        let nest_entity_id = format!("{}.hne", key_parent);

        let has_nest: bool = save_file.load_bool(&has_nest_key)?;
        let mut nest_entity: Option<EntityID> = None;
        if has_nest {
            let eid = save_file.load_u64(&nest_entity_id)?;
            nest_entity = Some(EntityID { id: eid });
        }

        let tm = TileMethods::OakTree(TileOakTree {
            has_nest: has_nest,
            nest_id: nest_entity,
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
