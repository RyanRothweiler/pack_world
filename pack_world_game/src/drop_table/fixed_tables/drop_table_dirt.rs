use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static DIRT: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::Dirt, 1), 10.0),
        (EntryOutput::new_tile(TileType::Dirt, 4), 5.0),
        (EntryOutput::new_tile(TileType::Dirt, 10), 1.0),
        (EntryOutput::new_tile(TileType::Dirt, 25), 0.1),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        DIRT.check_cycle(&plat_api);
    }
}
