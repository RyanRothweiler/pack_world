use crate::{
    drop_table::FixedTableID,
    item::ItemType,
    pack::{pack_id::PackID, Pack},
};
use elara_engine::vectors::*;
use std::sync::LazyLock;

pub static STICK: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Stick".into(),
        vec![(ItemType::Stick, 20)],
        4,
        FixedTableID::Pack(PackID::Stick),
        VecThreeFloat::new(-6.0, 0.0, 6.0),
    )
});
