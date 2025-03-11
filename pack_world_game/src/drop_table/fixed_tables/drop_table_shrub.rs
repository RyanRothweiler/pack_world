use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static SHRUB: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        ((EntryOutput::new_item(ItemType::Stick, 1), 1.0)),
        ((EntryOutput::new_item(ItemType::Berry, 1), 2.0)),
        ((EntryOutput::new_tile(TileType::BirdNest, 1), 0.1)),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        SHRUB.check_cycle(&plat_api);
    }
}
