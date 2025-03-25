use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static PACK_STARTER: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::Grass, 1), 14.0),
        (EntryOutput::new_tile(TileType::Dirt, 1), 25.0),
        (EntryOutput::new_tile(TileType::Boulder, 1), 5.0),
        (EntryOutput::new_tile(TileType::MudPit, 1), 5.0),
        (EntryOutput::new_gold(15), 0.5),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        PACK_STARTER.check_cycle(&plat_api);
    }
}
