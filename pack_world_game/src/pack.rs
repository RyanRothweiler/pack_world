use crate::{drop_table::*, state::inventory::*, tiles::*};
use rand::prelude::*;
use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PackID {
    Starter,
}

#[derive(Debug)]
pub struct Pack {
    pub display_name: String,
    pub cost: Vec<(ItemType, i32)>,
    pub table: DropTable,
}

impl Pack {
    pub fn new(display_name: String, cost: Vec<(ItemType, i32)>, table: DropTable) -> Pack {
        Pack {
            display_name,
            cost,
            table,
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

    pub fn pull(&self, inventory: &Inventory) -> Option<ItemType> {
        if !self.can_afford(inventory) {
            return None;
        }

        Some(self.table.pull())
    }
}

// TODO use lazylock here instead of this init method
pub static PACK_LIB: OnceLock<HashMap<PackID, Pack>> = OnceLock::new();

pub fn init_static_packs() {
    let mut pack_lib: HashMap<PackID, Pack> = HashMap::new();

    // Starter
    pack_lib.insert(
        PackID::Starter,
        Pack::new(
            "Starter".into(),
            vec![(ItemType::DirtClod, 5)],
            DropTable::new(vec![
                (ItemType::Tile(TileType::Dirt), 10.0),
                (ItemType::Tile(TileType::Grass), 10.0),
            ]),
        ),
    );

    PACK_LIB.set(pack_lib).unwrap();
}

pub fn get_pack_info(pack_id: PackID) -> &'static Pack {
    &PACK_LIB.get().unwrap().get(&pack_id).unwrap()
}
