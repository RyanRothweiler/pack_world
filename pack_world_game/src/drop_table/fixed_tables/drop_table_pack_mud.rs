use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static PACK_MUD: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::Grass, 1), 14.0),
        (EntryOutput::new_tile(TileType::Grass, 1), 14.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        PACK_MUD.check_cycle(&plat_api);
    }
}
