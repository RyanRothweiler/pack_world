use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static PACK_STARTER: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::Dirt, 1), 25.0),
        (EntryOutput::new_tile(TileType::Grass, 1), 14.0),
        (EntryOutput::new_tile(TileType::Boulder, 1), 5.0),
        (EntryOutput::new_gold(15), 0.5),
    ])
});

mod test {
    use super::*;

    #[test]
    pub fn check_cycle() {
        PACK_STARTER.check_cycle();
    }
}
