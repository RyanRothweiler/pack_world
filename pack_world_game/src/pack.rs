use crate::{drop_table::*, state::inventory::*, tiles::*};
use rand::prelude::*;
use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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

static STARTER: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Starter".into(),
        vec![(ItemType::DirtClod, 5)],
        DropTable::new(vec![
            (ItemType::Tile(TileType::Dirt), 10.0),
            (ItemType::Tile(TileType::Grass), 10.0),
        ]),
    )
});

pub fn get_pack_info(pack_id: PackID) -> &'static Pack {
    match pack_id {
        PackID::Starter => &STARTER,
    }
}
