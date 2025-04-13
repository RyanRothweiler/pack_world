use crate::{
    drop_table::*,
    error::*,
    grid::*,
    save_file::{load, *},
    update_signal::*,
    world::*,
};
use gengar_engine::{platform_api::*, vectors::*};

#[derive(Debug)]
pub struct HarvestTimer {
    // tile until we can harvest
    length: f64,
    time: f64,

    // modifies the length
    length_condition: Vec<LengthCondition>,

    // Adds drops
    drop_conditions: Vec<DropCondition>,

    pub table: FixedTableID,
}

/// Adjust the length based on condition
#[derive(Debug)]
struct LengthCondition {
    time_mod: f64,
    condition: WorldConditionState,
}

/// Add a drop based on conditino
#[derive(Debug)]
struct DropCondition {
    entry: (EntryOutput, f64),
    condition: WorldConditionState,
}

impl HarvestTimer {
    pub fn new(length: f64, table_id: FixedTableID) -> Self {
        Self {
            length,
            table: table_id,
            time: 0.0,
            length_condition: vec![],
            drop_conditions: vec![],
        }
    }

    pub fn add_length_condition(&mut self, time_mod: f64, condition: WorldCondition) {
        self.length_condition.push(LengthCondition {
            time_mod,
            condition: WorldConditionState::new(condition),
        });
    }

    pub fn add_drop_condition(&mut self, entry: (EntryOutput, f64), condition: WorldCondition) {
        self.drop_conditions.push(DropCondition {
            entry,
            condition: WorldConditionState::new(condition),
        })
    }

    pub fn update_world_conditions(&mut self, pos: GridPos, world_snapshot: &WorldSnapshot) {
        // length conditions
        for lc in &mut self.length_condition {
            lc.condition.update(pos, world_snapshot);
        }

        // drop conditions
        for dc in &mut self.drop_conditions {
            dc.condition.update(pos, world_snapshot);
        }
    }

    pub fn percent_done(&self) -> f64 {
        (self.time / self.length()).clamp(0.0, 1.0)
    }

    pub fn inc(&mut self, time: f64) {
        self.time += time;
        self.time = self.time.clamp(0.0, self.length());
    }

    pub fn can_harvest(&self) -> bool {
        self.time >= self.length()
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }

    #[must_use]
    pub fn harvest(&mut self, platform_api: &PlatformApi) -> Drop {
        let mut drop_table_instance = DropTableInstance::new_fixed(self.table);
        for dc in &self.drop_conditions {
            if dc.condition.is_affirm() {
                drop_table_instance = drop_table_instance.add_entry(dc.entry.clone());
            }
        }

        self.reset();
        return drop_table_instance.get_drop(platform_api);
    }

    pub fn length(&self) -> f64 {
        let mut mod_accum: f64 = 1.0;
        for lc in &self.length_condition {
            if lc.condition.is_affirm() {
                mod_accum += lc.time_mod;
            }
        }

        self.length * mod_accum
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let length_key = format!("{}.l", key_parent);
        let time_key = format!("{}.t", key_parent);

        save_file.save_f64(&length_key, self.length);
        save_file.save_f64(&time_key, self.time);

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let length_key = format!("{}.l", key_parent);
        let time_key = format!("{}.t", key_parent);

        let length = save_file
            .load_f64(&length_key)
            .expect(&format!("Misisng key {length_key}"));
        let time = save_file
            .load_f64(&time_key)
            .expect(&format!("Misisng key {length_key}"));

        let mut timer = Self::new(length, FixedTableID::Grass);
        timer.time = time;

        Ok(timer)
    }
}

/*
mod test {
    use super::*;

    #[test]
    fn tests() {
        let mut ht = HarvestTimer::new(10.0, FixedTableID::Boulder);

        assert_eq!(ht.can_harvest(), false);

        ht.inc(10.0);
        assert_eq!(ht.can_harvest(), true);

        ht.reset();
        assert_eq!(ht.can_harvest(), false);

        ht.set_length_mod(0.5);
        ht.inc(5.0);
        assert_eq!(ht.can_harvest(), true);
    }
}
*/
