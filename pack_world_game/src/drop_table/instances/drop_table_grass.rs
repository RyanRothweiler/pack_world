use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static GRASS: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_item(ItemType::DirtClod, 1), 10.0),
        (EntryOutput::new_item(ItemType::Stick, 1), 4.0),
    ])
});

mod test {
    use super::*;

    #[test]
    pub fn check_cycle() {
        GRASS.check_cycle();
    }
}
