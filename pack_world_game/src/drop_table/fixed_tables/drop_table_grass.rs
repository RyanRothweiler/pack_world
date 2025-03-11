use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static GRASS: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_item(ItemType::DirtClod, 1), 10.0),
        (EntryOutput::new_item(ItemType::Stick, 1), 4.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        GRASS.check_cycle(&plat_api);
    }
}
