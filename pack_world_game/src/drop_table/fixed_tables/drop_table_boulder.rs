use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static BOULDER: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        ((EntryOutput::new_item(ItemType::Rock, 1), 10.0)),
        ((EntryOutput::new_table(FixedTableID::SmallGold, 1), 3.0)),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        BOULDER.check_cycle(&plat_api);
    }
}
