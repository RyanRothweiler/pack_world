use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static BOULDER: LazyLock<DropTable> =
    LazyLock::new(|| DropTable::new(vec![((EntryOutput::new_item(ItemType::Rock, 1), 10.0))]));

mod test {
    use super::*;

    #[test]
    pub fn check_cycle() {
        BOULDER.check_cycle();
    }
}
