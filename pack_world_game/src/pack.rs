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
    pub table_id: DropTableID,

    pub content_count: i32,
}

impl Pack {
    pub fn new(
        display_name: String,
        cost: Vec<(ItemType, i64)>,
        content_count: i32,
        table_id: DropTableID,
    ) -> Pack {
        Pack {
            display_name,
            cost,
            table_id,
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
    pub fn pull(&self) -> Drop {
        get_drop(self.table_id)
        // Some(self.table.pull())
    }
}

static STARTER: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Starter".into(),
        vec![(ItemType::DirtClod, 5)],
        4,
        DropTableID::Grass,
    )
});
pub fn get_pack_info(pack_id: PackID) -> &'static Pack {
    match pack_id {
        PackID::Starter => &STARTER,
    }
}
