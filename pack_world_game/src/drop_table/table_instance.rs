pub use crate::drop_table::*;

#[derive(Clone)]
pub enum DropTableInstance {
    /// Pull from one of the fixed static tables
    Fixed(FixedTableID),

    /// Pull from a fully custom table
    Custom(DropTable),
}

impl DropTableInstance {
    pub fn new_fixed(table_id: FixedTableID) -> Self {
        Self::Fixed(table_id)
    }

    /// Convert the fixed table to a custom one (if needed), and add the new entry to the custom table
    #[must_use]
    pub fn add_entry(&self, input: (EntryOutput, f64)) -> DropTableInstance {
        // convert to custom table if needed
        let mut ret: DropTableInstance = match self {
            DropTableInstance::Fixed(table_id) => {
                let drop_table = get_fixed_table(*table_id);
                DropTableInstance::Custom(drop_table.clone())
            }

            DropTableInstance::Custom(_) => self.clone(),
        };

        // Add the entry to the new table
        match &mut ret {
            DropTableInstance::Custom(table) => table.add_entry(input),
            DropTableInstance::Fixed(_) => unreachable!(),
        }

        return ret;
    }

    pub fn get_drop(&self) -> Drop {
        match self {
            DropTableInstance::Fixed(table_id) => get_drop(*table_id),
            DropTableInstance::Custom(table) => {
                let mut tables_visited: Vec<FixedTableID> = vec![];
                table.pull(&mut tables_visited)
            }
        }
    }

    pub fn entries_count(&self) -> usize {
        match self {
            DropTableInstance::Fixed(table_id) => get_fixed_table(*table_id).entries.len(),
            DropTableInstance::Custom(table) => table.entries.len(),
        }
    }
}

mod test {
    use super::*;

    #[test]
    pub fn table_conversion() {
        let mut table = DropTableInstance::new_fixed(FixedTableID::TestGold);
        assert_eq!(table.entries_count(), 1);

        let drop = table.get_drop();
        assert_eq!(drop.amount, 1);
        assert_eq!(drop.drop_type, DropType::Gold);

        table = table.add_entry((EntryOutput::new_gold(1), 1.0));
        assert_eq!(table.entries_count(), 2);
    }
}
