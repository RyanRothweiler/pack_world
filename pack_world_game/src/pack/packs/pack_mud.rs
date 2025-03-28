use crate::{
    drop_table::FixedTableID,
    item::ItemType,
    pack::{pack_id::PackID, Pack},
};
use std::sync::LazyLock;

pub static MUD: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Mud".into(),
        vec![(ItemType::DirtClod, 10), (ItemType::Stick, 10)],
        4,
        FixedTableID::Pack(PackID::Mud),
    )
});
