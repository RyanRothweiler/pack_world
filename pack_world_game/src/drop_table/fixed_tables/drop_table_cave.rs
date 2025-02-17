use crate::{drop_table::*, item::*, pack::*};
use std::sync::LazyLock;

pub static CAVE: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_table(FixedTableID::SmallGold, 3), 25.0),
        (EntryOutput::new_item(ItemType::DragonEgg, 1), 0.1),
        (EntryOutput::new_item(ItemType::Baby, 1), 15.0),
    ])
});

mod test {
    use super::*;

    #[test]
    pub fn check_cycle() {
        CAVE.check_cycle();
    }
}
