use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static PACK_MUD: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::Dirt, 3), 10.0),
        (EntryOutput::new_tile(TileType::Water, 2), 10.0),
        (EntryOutput::new_tile(TileType::MudHenge, 1), 3.0),
        (EntryOutput::new_tile(TileType::Newt, 1), 1.0),
        (EntryOutput::new_tile(TileType::Reed, 1), 5.0),
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
