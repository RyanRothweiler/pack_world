use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static SMALL_GOLD: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        ((EntryOutput::new_gold(1), 10.0)),
        ((EntryOutput::new_gold(7), 4.0)),
        ((EntryOutput::new_gold(25), 1.0)),
        ((EntryOutput::new_gold(200), 0.1)),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        SMALL_GOLD.check_cycle(&plat_api);
    }
}
