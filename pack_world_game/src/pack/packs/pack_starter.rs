use crate::{
    drop_table::FixedTableID,
    item::ItemType,
    pack::{pack_id::PackID, Pack},
};
use elara_engine::vectors::*;
use std::sync::LazyLock;

pub static STARTER: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Starter".into(),
        vec![(ItemType::DirtClod, 5)],
        4,
        FixedTableID::Pack(PackID::Starter),
        VecThreeFloat::new(0.0, 0.0, 0.0),
    )
});
