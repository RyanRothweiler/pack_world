use crate::{
    drop_table::FixedTableID,
    item::ItemType,
    pack::{pack_id::PackID, Pack},
};
use gengar_engine::vectors::*;
use std::sync::LazyLock;

pub static MUD_HEART: LazyLock<Pack> = LazyLock::new(|| {
    Pack::new(
        "Mud Heart".into(),
        vec![(ItemType::MudHeart, 5)],
        4,
        FixedTableID::Pack(PackID::MudHeart),
        VecThreeFloat::new(0.0, 0.0, 18.0),
    )
});
