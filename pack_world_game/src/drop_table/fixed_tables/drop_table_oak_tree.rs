use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static OAK_TREE: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_item(ItemType::Stick, 1), 3.0),
        (EntryOutput::new_item(ItemType::OakLog, 1), 3.0),
        (EntryOutput::new_item(ItemType::Acorn, 1), 2.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        OAK_TREE.check_cycle(&plat_api);
    }
}
