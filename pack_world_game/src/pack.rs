use crate::{drop_table::*, error::*, item::*, save_file::*, state::inventory::*, tile::*};
use elara_engine::{platform_api::*, vectors::*};

pub mod pack_id;
pub mod packs;

pub use pack_id::PackID;
pub use packs::*;

#[derive(Debug)]
pub struct Pack {
    pub display_name: String,
    pub cost: Vec<(ItemType, i64)>,
    pub table_id: FixedTableID,

    // how many drops to pull from pack
    pub content_count: i32,

    pub shop_position: VecThreeFloat,
}

impl Pack {
    pub fn new(
        display_name: String,
        cost: Vec<(ItemType, i64)>,
        content_count: i32,
        table_id: FixedTableID,
        shop_position: VecThreeFloat,
    ) -> Pack {
        Pack {
            display_name,
            cost,
            table_id,
            content_count,
            shop_position,
        }
    }

    pub fn can_afford(&self, inventory: &Inventory) -> bool {
        for c in &self.cost {
            if !inventory.has_atleast(c.0, c.1) {
                return false;
            }
        }

        true
    }

    pub fn spend(&self, inventory: &mut Inventory) {
        if !self.can_afford(inventory) {
            eprintln!("Attempted to open pack that you cannot afford.");
            return;
        }

        for c in &self.cost {
            inventory.give_item(c.0, -c.1).expect("Error spending item");
        }
    }

    // Assumes you can afford the pack
    pub fn pull(&self, platform_api: &PlatformApi) -> Drop {
        get_drop(self.table_id, platform_api)
    }
}
