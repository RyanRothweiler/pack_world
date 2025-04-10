use crate::{grid::*, tile::*, world::*};

/// Static tile info
pub struct TileDefinition {
    pub title: &'static str,
    pub description: &'static str,
    pub world_layer: WorldLayer,
    pub footprint: Vec<GridPos>,
    pub placement_constraints: Vec<WorldCondition>,

    pub build_methods: fn(origin: GridPos) -> TileMethods,
}
