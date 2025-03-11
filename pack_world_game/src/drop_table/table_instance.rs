use crate::drop_table::*;
use gengar_engine::platform_api::*;

/// A specific 'reference' to a drop table. Can be easily converted between the options.
#[derive(Clone, Debug)]
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

    pub fn get_drop(&self, platform_api: &PlatformApi) -> Drop {
        match self {
            DropTableInstance::Fixed(table_id) => get_drop(*table_id, platform_api),
            DropTableInstance::Custom(table) => {
                let mut tables_visited: Vec<FixedTableID> = vec![];
                table.pull(&mut tables_visited, platform_api)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    #[test]
    pub fn table_conversion() {
        let plat_api = windows_plaform_api();

        let mut table = DropTableInstance::new_fixed(FixedTableID::TestGold);
        assert_eq!(table.entries_count(), 1);

        let drop = table.get_drop(&plat_api);
        assert_eq!(drop.amount, 1);
        assert_eq!(drop.drop_type, DropType::Gold);

        table = table.add_entry((EntryOutput::new_gold(1), 1.0));
        assert_eq!(table.entries_count(), 2);
    }
}
