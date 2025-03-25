use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static MUD_PIT: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_tile(TileType::Dirt, 1), 25.0),
        (EntryOutput::new_tile(TileType::Dirt, 1), 25.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        MUD_PIT.check_cycle(&plat_api);
    }
}
