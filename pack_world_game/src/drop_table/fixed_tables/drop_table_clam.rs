use crate::{drop_table::*, item::*, pack::*};
use std::sync::LazyLock;

pub static CLAM: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_table(FixedTableID::SmallGold, 1), 1.0),
        (EntryOutput::new_item(ItemType::OldHat, 1), 5.0),
        (EntryOutput::new_item(ItemType::Seaweed, 1), 5.0),
        (EntryOutput::new_item(ItemType::TrashBag, 1), 5.0),
        (EntryOutput::new_item(ItemType::OldBoot, 1), 5.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        CLAM.check_cycle(&windows_plaform_api());
    }
}
