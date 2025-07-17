use crate::grid::GridPos;

/// "Radius" iterator.
/// Origin is included. So origin 0, radius 2, would be -2, -1, 0, 1, 2
/// This is a manhattan distance radius
pub struct GridPosRadiusIter {
    pos: GridPos,

    x: i32,
    y: i32,

    radius: i32,
}

impl GridPosRadiusIter {
    pub fn new(pos: GridPos, radius: i32) -> Self {
        assert!(radius > 0, "Radius must be positive");

        Self {
            pos,
            radius,
            x: -radius,
            y: -radius,
        }
    }
}

impl Iterator for GridPosRadiusIter {
    type Item = GridPos;

    fn next(&mut self) -> Option<GridPos> {
        if self.y > self.radius {
            return None;
        }

        let res = self.pos + GridPos::new(self.x, self.y);

        self.x += 1;
        if self.x > self.radius {
            self.x = -self.radius;
            self.y += 1;
        }

        Some(res)
    }
}

mod test {
    use super::*;

    #[test]
    fn iter_test() {
        let grid_pos = GridPos::new(0, 0);
        let adjs: Vec<GridPos> = grid_pos.to_radius_iter(2).collect();

        assert_eq!(adjs.len(), 25);

        assert!(adjs.contains(&GridPos::new(-2, -2)));
        assert!(adjs.contains(&GridPos::new(-1, -2)));
        assert!(adjs.contains(&GridPos::new(0, -2)));
        assert!(adjs.contains(&GridPos::new(1, -2)));
        assert!(adjs.contains(&GridPos::new(2, -2)));

        assert!(adjs.contains(&GridPos::new(-1, -1)));
        assert!(adjs.contains(&GridPos::new(-2, -1)));
        assert!(adjs.contains(&GridPos::new(0, -1)));
        assert!(adjs.contains(&GridPos::new(1, -1)));
        assert!(adjs.contains(&GridPos::new(2, -1)));

        assert!(adjs.contains(&GridPos::new(-1, 0)));
        assert!(adjs.contains(&GridPos::new(-2, 0)));
        assert!(adjs.contains(&GridPos::new(0, 0)));
        assert!(adjs.contains(&GridPos::new(1, 0)));
        assert!(adjs.contains(&GridPos::new(2, 0)));

        assert!(adjs.contains(&GridPos::new(-1, 1)));
        assert!(adjs.contains(&GridPos::new(-2, 1)));
        assert!(adjs.contains(&GridPos::new(0, 1)));
        assert!(adjs.contains(&GridPos::new(1, 1)));
        assert!(adjs.contains(&GridPos::new(2, 1)));

        assert!(adjs.contains(&GridPos::new(-1, 2)));
        assert!(adjs.contains(&GridPos::new(-2, 2)));
        assert!(adjs.contains(&GridPos::new(0, 2)));
        assert!(adjs.contains(&GridPos::new(1, 2)));
        assert!(adjs.contains(&GridPos::new(2, 2)));
    }
}
