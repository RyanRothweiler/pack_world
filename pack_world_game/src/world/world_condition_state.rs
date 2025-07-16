use crate::world::*;

/// Holds state of a world condition

#[derive(Debug)]
pub struct WorldConditionState {
    affirm: bool,
    pub condition: WorldCondition,
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
        self.affirm = self.condition.valid(grid_pos, world_snapshot);
    }
}

mod test {
    use super::*;

    #[test]
    fn state_updating() {
        let mut world = World::new();

        let _ = world.insert_tile_type(GridPos::new(0, 0), TileType::Dirt);
        let _ = world.insert_tile_type(GridPos::new(0, 1), TileType::Water);
        let _ = world.insert_tile_type(GridPos::new(10, 10), TileType::Grass);

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
