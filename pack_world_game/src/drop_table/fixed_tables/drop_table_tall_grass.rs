use crate::{drop_table::*, item::*, pack::*};
use std::sync::LazyLock;

pub static TALL_GRASS: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_table(FixedTableID::SmallGold, 3), 25.0),
        // (EntryOutput::new_item(ItemType::Baby, 1), 15.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        TALL_GRASS.check_cycle(&plat_api);
    }
}
