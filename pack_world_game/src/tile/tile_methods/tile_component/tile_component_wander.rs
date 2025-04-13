use crate::grid::*;
use gengar_engine::{platform_api::*, vectors::*};

const MOVE_SPEED: f64 = 0.5;

pub struct WanderState {
    pub curr_world_pos: VecTwo,
    pub target_grid_offset: GridPos,
}

impl WanderState {
    pub fn update(&mut self, origin: GridPos, time_step: f64, platform_api: &PlatformApi) {
        let target_world = grid_to_world(&(origin + self.target_grid_offset));
        let mut dir = target_world - self.curr_world_pos;
        dir.normalize();

        self.curr_world_pos = self.curr_world_pos + (dir * MOVE_SPEED);

        if self.curr_world_pos.dist_from(target_world) < 1.0 {
            self.target_grid_offset.x = ((platform_api.rand)() * 4.0) as i32;
            self.target_grid_offset.y = ((platform_api.rand)() * 4.0) as i32;
        }
    }
}
