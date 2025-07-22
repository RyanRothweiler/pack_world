use crate::{drop_table::*, item::*, pack::*};
use std::sync::LazyLock;

pub static NEWT: LazyLock<DropTable> =
    LazyLock::new(|| DropTable::new(vec![(EntryOutput::new_item(ItemType::EyeOfNewt, 1), 1.0)]));

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        NEWT.check_cycle(&plat_api);
    }
}
