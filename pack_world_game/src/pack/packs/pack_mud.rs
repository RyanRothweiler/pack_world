use crate::{
    drop_table::FixedTableID,
    item::ItemType,
    pack::{pack_id::PackID, Pack},
};
use elara_engine::vectors::*;
use std::sync::LazyLock;

pub static MUD: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Mud".into(),
        vec![(ItemType::DirtClod, 20)],
        4,
        FixedTableID::Pack(PackID::Mud),
        VecThreeFloat::new(0.0, 0.0, 6.0),
    )
});
