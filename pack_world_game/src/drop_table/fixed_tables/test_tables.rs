use crate::drop_table::*;

pub static TEST_CYCLE_A: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![(
        EntryOutput::new_table(FixedTableID::TestCycleB, 1),
        1.0,
    )])
});

pub static TEST_CYCLE_B: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![(
        EntryOutput::new_table(FixedTableID::TestCycleA, 1),
        1.0,
    )])
});

pub static TEST_GOLD: LazyLock<DropTable> =
    LazyLock::new(|| DropTable::new(vec![(EntryOutput::new_gold(1), 1.0)]));

pub static TEST_TABLE: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![(
        EntryOutput::new_table(FixedTableID::TestGold, 1),
        6.0,
    )])
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    #[should_panic]
    fn check_cycle() {
        let plat_api = windows_plaform_api();
        TEST_CYCLE_A.check_cycle(&plat_api);
    }
}
