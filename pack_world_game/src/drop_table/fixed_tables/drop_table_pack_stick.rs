use crate::{drop_table::*, item::*, pack::*};
use std::sync::LazyLock;

pub static PACK_STICK: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::OakTree, 1), 1.0),
        (EntryOutput::new_tile(TileType::Boulder, 1), 1.0),
        (EntryOutput::new_tile(TileType::BirdNest, 1), 1.0),
        (EntryOutput::new_tile(TileType::Cave, 1), 1.0),
        (EntryOutput::new_tile(TileType::Shrub, 1), 1.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        PACK_STICK.check_cycle(&plat_api);
    }
}
