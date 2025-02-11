use crate::{drop_table::*, item::*, state::inventory::*, tile::*};
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
    pub cost: Vec<(ItemType, i64)>,
    pub table: DropTable,

    pub content_count: i32,
}

impl Pack {
    pub fn new(
        display_name: String,
        cost: Vec<(ItemType, i64)>,
        content_count: i32,
        table: DropTable,
    ) -> Pack {
        Pack {
            display_name,
            cost,
            table,
            content_count,
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

    // Assumes you can afford the pack
    pub fn pull(&self) -> Option<Drop> {
        Some(self.table.pull())
    }
}

static STARTER: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Starter".into(),
        vec![(ItemType::DirtClod, 5)],
        4,
        DropTable::new(vec![
            (Drop::new_tile(TileType::Dirt, 1), 25.0),
            (Drop::new_tile(TileType::Grass, 1), 12.0),
            (Drop::new_tile(TileType::Boulder, 1), 8.0),
            (Drop::new_gold(15), 0.5),
        ]),
    )
});

pub fn get_pack_info(pack_id: PackID) -> &'static Pack {
    match pack_id {
        PackID::Starter => &STARTER,
    }
}
