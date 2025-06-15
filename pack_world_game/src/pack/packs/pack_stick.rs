use crate::{
    drop_table::FixedTableID,
    item::ItemType,
    pack::{pack_id::PackID, Pack},
};
use gengar_engine::vectors::*;
use std::sync::LazyLock;

pub static STICK: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Stick".into(),
        vec![(ItemType::Stick, 20)],
        4,
        FixedTableID::Pack(PackID::Stick),
        VecThreeFloat::new(-9.0, 0.0, 9.0),
    )
});
