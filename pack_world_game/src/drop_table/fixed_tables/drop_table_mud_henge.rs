use crate::{drop_table::*, pack::*};
use std::sync::LazyLock;

pub static MUD_HENGE: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (EntryOutput::new_item(ItemType::MudHeart, 1), 10.0),
        (EntryOutput::new_item(ItemType::MudHeart, 2), 5.0),
        (EntryOutput::new_item(ItemType::MudHeart, 3), 1.0),
    ])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn check_cycle() {
        let plat_api = windows_plaform_api();
        MUD_HENGE.check_cycle(&plat_api);
    }
}
