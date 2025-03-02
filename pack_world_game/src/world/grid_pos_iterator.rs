// Attepmt at a lending iterator.
// I didn't know what a lending iterator was when I started writing this
// So this doesn't actually work. Just keeping around for now.

use crate::{grid::*, tile::*, world::*};

/// For iterating through all entities at one grid position
pub struct GridPosIterator<'a> {
    pub world: &'a mut World,
    pub eids: Vec<EntityID>,
    pub i: usize,
}

impl<'a> Iterator for GridPosIterator<'a> {
    type Item = &'a mut TileInstance;

    fn next(&mut self) -> Option<&'a mut TileInstance> {
        if let Some(eid) = self.eids.get(self.i) {
            self.i += 1;
            Some(
                self.world
                    .entities
                    .get_mut(eid)
                    .expect("All entity ids should be valid"),
            )
        } else {
            None
        }
    }
}

mod test {
    use super::*;

    // #[test]
    pub fn grid_pos_iter() {
        let mut world = World::new();

        let _ = world.force_insert_tile(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(0, 0), TileType::Grass);
        let _ = world.force_insert_tile(GridPos::new(0, 1), TileType::Dirt);
        let _ = world.force_insert_tile(GridPos::new(1, 1), TileType::Dirt);

        for tile_inst in world.grid_pos_iter(GridPos::new(0, 0)) {}
    }
}
