use crate::{
    drop_table::FixedTableID,
    item::ItemType,
    pack::{pack_id::PackID, Pack},
};
use gengar_engine::vectors::*;
use std::sync::LazyLock;

pub static WATER: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Water".into(),
        vec![(ItemType::Dew, 10)],
        4,
        FixedTableID::Pack(PackID::Water),
        VecThreeFloat::new(9.0, 0.0, 9.0),
    )
});
