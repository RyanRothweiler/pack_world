use crate::{grid::*, tile::*, world::*};

/// Static tile info
pub struct TileDefinition {
    pub title: &'static str,
    pub description: &'static str,

    pub world_layer: WorldLayer,

    /// if true then when placing the tile this will draw at every footprint position
    /// instead of just the origin.
    pub placing_draw_footprint: bool,

    pub footprint: Vec<GridPos>,

    // Restrictions on where this can be placed
    pub placement_constraints: Vec<WorldCondition>,

    // When placed, apply these global modifications
    pub placement_global_mod: Vec<GlobalMod>,

    pub new_instance: fn(grid_pos: GridPos) -> TileInstance,
}
