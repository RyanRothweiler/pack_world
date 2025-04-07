use crate::{grid::*, tile_type::TileType, world::*};

#[derive(Debug)]
pub struct WorldConditionState {
    affirm: bool,
    pub condition: WorldCondition,
}

#[derive(Debug)]
pub enum WorldCondition {
    AdjacentTo(TileSnapshot),
}

impl WorldConditionState {
    pub fn new(condition: WorldCondition) -> Self {
        Self {
            affirm: false,
            condition,
        }
    }

    pub fn is_affirm(&self) -> bool {
        self.affirm
    }

    pub fn update(&mut self, grid_pos: GridPos, world_snapshot: &WorldSnapshot) {
        self.affirm = false;

        match self.condition {
            WorldCondition::AdjacentTo(ty) => {
                for adj_pos in grid_pos.to_adjacents_iter() {
                    if let Some(world_cell) = world_snapshot.entity_map.get(&adj_pos) {
                        for (layer, eid) in &world_cell.layers {
                            let tile = world_snapshot.entities.get(&eid).unwrap();
                            if *tile == ty {
                                self.affirm = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn adjacent_to() {
        let mut world = World::new();

        let _ = world.insert_tile(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.insert_tile(GridPos::new(0, 1), TileType::Water);
        let _ = world.insert_tile(GridPos::new(10, 10), TileType::Grass);

        let snapshot = world.get_world_snapshot();

        let mut cond = WorldConditionState::new(WorldCondition::AdjacentTo(TileSnapshot::Water));
        assert_eq!(cond.affirm, false);

        cond.update(GridPos::new(0, 0), &snapshot);
        assert_eq!(cond.affirm, true);

        cond.update(GridPos::new(0, 1), &snapshot);
        assert_eq!(cond.affirm, false);

        cond.update(GridPos::new(10, 0), &snapshot);
        assert_eq!(cond.affirm, false);

        cond.update(GridPos::new(10, 10), &snapshot);
        assert_eq!(cond.affirm, false);
    }
}
