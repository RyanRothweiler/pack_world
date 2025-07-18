use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static PACK_WATER: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::Water, 1), 3.0),
        (EntryOutput::new_tile(TileType::Clam, 1), 1.0),
        (EntryOutput::new_tile(TileType::Cave, 1), 1.0),
        (EntryOutput::new_tile(TileType::Spring, 1), 1.0),
        (EntryOutput::new_tile(TileType::Kelp, 1), 1.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        PACK_WATER.check_cycle(&plat_api);
    }
}
