use crate::vectors::*;

/// Ray cast v plane intersection. Returns distance of ray
pub fn plane_intersection_distance(
    ray_origin: VecThreeFloat,
    ray_dir: VecThreeFloat,
    plane_center: VecThreeFloat,
    plane_normal: VecThreeFloat,
) -> Option<f64> {
    let denom = VecThreeFloat::dot(&plane_normal, &ray_dir);
    if denom > 1e-6 {
        let p = plane_center - ray_origin;
        let t = VecThreeFloat::dot(&p, &plane_normal) / denom;
        return Some(t);
    }

    // ray is parallel to the plane
    return None;
}

pub fn point_within_circle(pos: VecTwo, circle_center: VecTwo, radius: f64) -> bool {
    let dist = pos.dist_from(circle_center);
    dist <= radius
}

mod test {
    use super::*;

    #[test]
    pub fn plane_intersection_distance() {
        let d = super::plane_intersection_distance(
            VecThreeFloat::new(0.0, 0.0, 10.0),
            VecThreeFloat::new(0.0, 0.0, -1.0),
            VecThreeFloat::new(0.0, 0.0, 0.0),
            VecThreeFloat::new(0.0, 0.0, -1.0),
        );
        assert_eq!(d, Some(10.0));
    }

    #[test]
    pub fn point_within_circle() {
        assert!(super::point_within_circle(
            VecTwo::new(2.0, 2.0),
            VecTwo::new(5.0, 5.0),
            10.0,
        ));

        assert!(!super::point_within_circle(
            VecTwo::new(2.0, 2.0),
            VecTwo::new(5.0, 5.0),
            1.0,
        ));
    }
}
