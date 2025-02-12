use crate::drop_table::*;

pub static TEST_CYCLE_A: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![(
        EntryOutput::new_table(DropTableID::TestCycleB, 1),
        1.0,
    )])
});

pub static TEST_CYCLE_B: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![(
        EntryOutput::new_table(DropTableID::TestCycleA, 1),
        1.0,
    )])
});

pub static TEST_GOLD: LazyLock<DropTable> =
    LazyLock::new(|| DropTable::new(vec![(EntryOutput::new_gold(1), 1.0)]));

pub static TEST_TABLE: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![(
        EntryOutput::new_table(DropTableID::TestGold, 1),
        6.0,
    )])
});

mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn check_cycle() {
        TEST_CYCLE_A.check_cycle();
    }
}
