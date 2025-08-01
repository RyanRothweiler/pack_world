use elara_engine::vectors::*;

mod grid_pos;

pub use grid_pos::*;

pub const GRID_SIZE: f64 = 2.0;

pub fn grid_snap(pos: &VecTwo) -> VecTwo {
    VecTwo::new(
        (pos.x / GRID_SIZE).round() * GRID_SIZE,
        (pos.y / GRID_SIZE).round() * GRID_SIZE,
    )
}

// technically kinda screen to grid
pub fn world_to_grid(pos: &VecTwo) -> GridPos {
    let mouse_grid = grid_snap(pos);
    return GridPos {
        x: (mouse_grid.x / GRID_SIZE) as i32,
        y: (mouse_grid.y / GRID_SIZE) as i32,
    };
}

pub fn grid_to_world(pos: &GridPos) -> VecThreeFloat {
    VecThreeFloat {
        x: pos.x as f64 * GRID_SIZE,
        y: 0.0,
        z: pos.y as f64 * GRID_SIZE,
    }
}
