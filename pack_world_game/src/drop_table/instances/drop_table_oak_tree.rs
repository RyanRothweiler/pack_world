use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static OAK_TREE: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_item(ItemType::Stick, 1), 3.0),
        (EntryOutput::new_item(ItemType::OakLog, 1), 3.0),
        (EntryOutput::new_item(ItemType::Acorn, 1), 2.0),
    ])
});

mod test {
    use super::*;

    #[test]
    pub fn check_cycle() {
        OAK_TREE.check_cycle();
    }
}
