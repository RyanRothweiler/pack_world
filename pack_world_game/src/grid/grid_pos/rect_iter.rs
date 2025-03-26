use crate::grid::GridPos;

/// This includes the origin point itself.
/// A Width and Height of 1 will return just the origin point.

pub struct GridPosRectIter {
    pos: GridPos,

    x: i32,
    y: i32,

    w: i32,
    h: i32,
}

impl GridPosRectIter {
    pub fn new(pos: GridPos, w: i32, h: i32) -> Self {
        assert!(w > 0 && h > 0, "Width and height must be positive");

        Self {
            pos,
            w,
            h,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for GridPosRectIter {
    type Item = GridPos;

    fn next(&mut self) -> Option<GridPos> {
        if self.y >= self.h {
            return None;
        }

        let res = self.pos + GridPos::new(self.x, self.y);

        self.x += 1;
        if self.x >= self.w {
            self.x = 0;
            self.y += 1;
        }

        Some(res)
    }
}

mod test {
    use super::*;

    #[test]
    fn iter_test() {
        let grid_pos = GridPos::new(10, 10);
        let adjs: Vec<GridPos> = grid_pos.to_rect_iter(2, 4).collect();

        assert_eq!(adjs.len(), 8);
        assert!(adjs.contains(&GridPos::new(10, 10)));
        assert!(adjs.contains(&GridPos::new(11, 10)));

        assert!(adjs.contains(&GridPos::new(10, 11)));
        assert!(adjs.contains(&GridPos::new(11, 11)));

        assert!(adjs.contains(&GridPos::new(10, 12)));
        assert!(adjs.contains(&GridPos::new(11, 12)));

        assert!(adjs.contains(&GridPos::new(10, 13)));
        assert!(adjs.contains(&GridPos::new(11, 13)));
    }
}
