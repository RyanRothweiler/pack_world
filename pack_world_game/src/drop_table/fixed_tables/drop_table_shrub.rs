use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static SHRUB: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        ((EntryOutput::new_item(ItemType::Stick, 1), 1.0)),
        ((EntryOutput::new_item(ItemType::Berry, 1), 2.0)),
        ((EntryOutput::new_tile(TileType::BirdNest, 1), 0.1)),
    ])
});

mod test {
    use super::*;

    #[test]
    pub fn check_cycle() {
        SHRUB.check_cycle();
    }
}
