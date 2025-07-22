use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static PACK_MUD_HEART: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::MudFish, 1), 1.0),
        (EntryOutput::new_tile(TileType::MudChicken, 1), 1.0),
        (EntryOutput::new_tile(TileType::Goblin, 1), 1.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        PACK_MUD_HEART.check_cycle(&plat_api);
    }
}
