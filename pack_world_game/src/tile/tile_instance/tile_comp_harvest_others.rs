use crate::{grid::*, update_signal::*, world::*};
use gengar_engine::{platform_api::*, time::*, vectors::*};

/// Automatically harvest someone else
#[derive(Debug)]
pub struct TileCompHarvestOthers {
    timer: Time,
    len: Time,
}

impl TileCompHarvestOthers {
    pub fn new(timer: Time) -> Self {
        Self {
            timer: Time::new(TimeUnit::MilliSeconds(0.0)),
            len: timer,
        }
    }

    #[must_use]
    pub fn update(
        &mut self,
        time_step: Time,
        grid_pos: &GridPos,
        world_snapshot: &WorldSnapshot,
    ) -> Vec<UpdateSignal> {
        self.timer = self.timer + time_step;
        self.timer.clamp_ms(0.0, self.len.as_milliseconds().value());

        let mut ret: Vec<UpdateSignal> = vec![];

        if self.timer.as_milliseconds().value() >= self.len.as_milliseconds().value() {
            // TODO this harvests adjacent, in the future to add more locations, expand GlobalModLocation and use that everywhere.
            // Add GlobalModLocation to resolve the locations.
            for adj in grid_pos.to_adjacents_iter() {
                if let Some(info) = world_snapshot.entity_harvest_perc.get(&adj) {
                    if info.1 >= 1.0 {
                        ret.push(UpdateSignal::TryHarvestTile { entity_id: info.0 });
                        self.timer = Time::new(TimeUnit::MilliSeconds(0.0));
                    }
                }
            }
        }

        ret
    }

    pub fn perc_done(&self) -> f64 {
        self.timer.as_milliseconds().value() / self.len.as_milliseconds().value()
    }
}
