use crate::{drop_table::*, item::*, pack::*};
use std::sync::LazyLock;

pub static FROG: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![(
        EntryOutput::new_table(FixedTableID::SmallGold, 3),
        25.0,
    )])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        FROG.check_cycle(&plat_api);
    }
}
