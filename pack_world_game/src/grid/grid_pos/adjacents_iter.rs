use crate::grid::GridPos;

pub struct GridPosAdjacentsIter {
    pub pos: GridPos,
    pub i: i32,
}

impl Iterator for GridPosAdjacentsIter {
    type Item = GridPos;

    fn next(&mut self) -> Option<GridPos> {
        let result = match self.i {
            0 => self.pos + GridPos::new(-1, 1),
            1 => self.pos + GridPos::new(-1, 0),
            2 => self.pos + GridPos::new(-1, -1),
            3 => self.pos + GridPos::new(0, 1),
            4 => self.pos + GridPos::new(0, -1),
            5 => self.pos + GridPos::new(1, 1),
            6 => self.pos + GridPos::new(1, 0),
            7 => self.pos + GridPos::new(1, -1),
            _ => return None,
        };

        self.i += 1;
        Some(result)
    }
}

mod test {
    use super::*;

    #[test]
    fn adjacent_iter() {
        let grid_pos = GridPos::new(10, 10);
        let adjs: Vec<GridPos> = grid_pos.to_adjacents_iter().collect();

        assert_eq!(adjs.len(), 8);
        assert!(adjs.contains(&GridPos::new(9, 9)));
        assert!(adjs.contains(&GridPos::new(9, 10)));
        assert!(adjs.contains(&GridPos::new(9, 11)));

        assert!(adjs.contains(&GridPos::new(10, 9)));
        assert!(adjs.contains(&GridPos::new(10, 11)));

        assert!(adjs.contains(&GridPos::new(11, 9)));
        assert!(adjs.contains(&GridPos::new(11, 10)));
        assert!(adjs.contains(&GridPos::new(11, 11)));
    }
}
