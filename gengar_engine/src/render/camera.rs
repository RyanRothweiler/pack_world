use crate::{
    ascii::*, color::*, debug::*, input::*, matricies::matrix_four_four::*, transform::*,
    vectors::*,
};

pub enum ProjectionType {
    Perspective { focal_length: f64 },
    Orthographic,
}

pub struct Camera {
    pub forward: VecThreeFloat,
    pub yaw: f64,
    pub pitch: f64,

    pub transform: Transform,

    pub view_mat: M44,
    pub view_mat_inverse: M44,

    pub projection_mat: M44,
    pub projection_mat_inverse: M44,

    pub projection_type: ProjectionType,
    pub resolution: VecTwo,
    pub near_plane: f64,
    pub far_plane: f64,
    pub fov: f64,
}

impl Camera {
    pub fn new(projection_type: ProjectionType, resolution: VecTwo) -> Self {
        Camera {
            transform: Transform::new(),

            view_mat: M44::new_identity(),
            view_mat_inverse: M44::new_identity(),

            projection_mat: M44::new_identity(),
            projection_mat_inverse: M44::new_identity(),

            forward: VecThreeFloat::new(0.0, 0.0, 1.0),

            resolution,
            projection_type,

            near_plane: 0.1,
            far_plane: 10.0,
            fov: 0.0,

            yaw: 90.0,
            pitch: 0.0,
        }
    }

    pub fn update_matricies(&mut self) {
        match &self.projection_type {
            ProjectionType::Perspective { focal_length } => {
                let aspect: f64 = self.resolution.x / self.resolution.y;
                let fov_y: f64 = 45.0;

                let mut f: f64 = fov_y.to_radians();

                f = f / 2.0;
                f = 1.0 / f.tan();

                let top = self.near_plane * f;
                let right = top * aspect;

                let a = (1.0 / aspect) * f;
                let b = f;

                let d: f64 =
                    (self.far_plane + self.near_plane) / (self.near_plane - self.far_plane);
                let e: f64 =
                    (2.0 * self.far_plane * self.near_plane) / (self.near_plane - self.far_plane);

                let a = 1.0;
                let b = aspect;
                let c = focal_length;

                // let a = 1.0 * focal_length;
                // let b = aspect * focal_length;

                self.projection_mat = M44::new_identity();
                self.projection_mat.set(0, 0, a * c);
                self.projection_mat.set(1, 1, b * c);
                self.projection_mat.set(2, 2, d);
                self.projection_mat.set(3, 2, e);
                self.projection_mat.set(2, 3, -1.0);
                self.projection_mat.set(3, 3, 0.0);

                /*
                self.projection_mat_inverse = M44::new_empty();
                self.projection_mat_inverse.set(0, 0, 1.0 / a);
                self.projection_mat_inverse.set(1, 1, 1.0 / b);
                self.projection_mat_inverse.set(3, 2, -1.0);
                self.projection_mat_inverse.set(2, 3, 1.0 / d);
                self.projection_mat_inverse.set(3, 3, c / d);
                */
            }

            ProjectionType::Orthographic => {
                // let width_half: f64 = self.resolution.x * 0.5;
                // let height_half: f64 = self.resolution.y * 0.5;

                let width: f64 = self.resolution.x;
                let height: f64 = self.resolution.y;

                let left: f64 = 0.0;
                let right: f64 = width;
                let top: f64 = 0.0;
                let bottom: f64 = height;

                let tx = -((right + left) / (right - left));
                let ty = -((top + bottom) / (top - bottom));
                let tz = -((self.far_plane + self.near_plane) / (self.far_plane - self.near_plane));

                self.projection_mat = M44::new_identity();
                self.projection_mat.set(0, 0, 2.0 / (right - left));
                self.projection_mat.set(1, 1, 2.0 / (top - bottom));
                self.projection_mat
                    .set(2, 2, -2.0 / (self.far_plane - self.near_plane));

                self.projection_mat.set(3, 0, tx);
                self.projection_mat.set(3, 1, ty);
                self.projection_mat.set(3, 2, tz);
                self.projection_mat.set(3, 3, 1.0);
            }
        }

        // view matrix
        {
            let up = VecThreeFloat::new(0.0, 1.0, 0.0);

            // Cam yaw / pitch axis
            self.forward = VecThreeFloat::new_zero();
            self.forward.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
            self.forward.y = self.pitch.to_radians().sin();
            self.forward.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
            self.forward.normalize();

            let target_pos = self.transform.local_position + (self.forward * -1.0);

            let mut cam_dir = self.transform.local_position - target_pos;
            cam_dir.normalize();

            let mut cam_right = VecThreeFloat::cross(up, cam_dir);
            cam_right.normalize();

            let cam_up = VecThreeFloat::cross(cam_dir, cam_right);

            // Setup matrix
            self.view_mat = M44::new_identity();

            let inv_pos = VecThreeFloat::new(
                -self.transform.local_position.x,
                -self.transform.local_position.y,
                -self.transform.local_position.z,
            );

            self.view_mat.set(0, 0, cam_right.x);
            self.view_mat.set(1, 0, cam_right.y);
            self.view_mat.set(2, 0, cam_right.z);

            self.view_mat.set(0, 1, cam_up.x);
            self.view_mat.set(1, 1, cam_up.y);
            self.view_mat.set(2, 1, cam_up.z);

            self.view_mat.set(0, 2, cam_dir.x);
            self.view_mat.set(1, 2, cam_dir.y);
            self.view_mat.set(2, 2, cam_dir.z);

            self.view_mat.translate(inv_pos);

            // view inverse
            {
                let ix = cam_right / (cam_right.length() * cam_right.length());
                let iy = cam_up / (cam_up.length() * cam_up.length());
                let iz = cam_dir / (cam_dir.length() * cam_dir.length());

                let mut ip = VecThreeFloat::new_zero();
                ip.x = (self.transform.local_position.x * ix.x)
                    + (self.transform.local_position.y * iy.x)
                    + (self.transform.local_position.z * iz.x);

                ip.y = (self.transform.local_position.x * ix.y)
                    + (self.transform.local_position.y * iy.y)
                    + (self.transform.local_position.z * iz.y);

                ip.z = (self.transform.local_position.x * ix.z)
                    + (self.transform.local_position.y * iy.z)
                    + (self.transform.local_position.z * iz.z);

                self.view_mat_inverse = M44::new_identity();
                self.view_mat_inverse.set(0, 0, ix.x);
                self.view_mat_inverse.set(1, 0, ix.y);
                self.view_mat_inverse.set(2, 0, ix.z);

                self.view_mat_inverse.set(0, 1, iy.x);
                self.view_mat_inverse.set(1, 1, iy.y);
                self.view_mat_inverse.set(2, 1, iy.z);

                self.view_mat_inverse.set(0, 2, iz.x);
                self.view_mat_inverse.set(1, 2, iz.y);
                self.view_mat_inverse.set(2, 2, iz.z);

                self.view_mat_inverse.set(0, 3, ip.x * -1.0);
                self.view_mat_inverse.set(1, 3, ip.y * -1.0);
                self.view_mat_inverse.set(2, 3, ip.z * -1.0);
            }
        }
    }

    // Control the camera as a fly-cam
    // Mouse for rotation and wasd for camera relative movement
    pub fn move_fly(&mut self, mov_speed: f64, input: &Input) {
        if input.mouse.button_right.pressing {
            let sens = 0.08;
            self.yaw = self.yaw - (input.mouse.pos_delta.x * sens);
            self.pitch = self.pitch - (input.mouse.pos_delta.y * sens);
        }

        let mut right = VecThreeFloat::cross(self.forward, VecThreeFloat::new(0.0, 1.0, 0.0));
        right.normalize();

        let mut up = VecThreeFloat::cross(self.forward, right);
        up.normalize();

        if input.get_key(KeyCode::A).pressing {
            self.transform.local_position = self.transform.local_position + (right * mov_speed);
        }
        if input.get_key(KeyCode::D).pressing {
            self.transform.local_position = self.transform.local_position - (right * mov_speed);
        }
        if input.get_key(KeyCode::S).pressing {
            self.transform.local_position =
                self.transform.local_position + (self.forward * mov_speed);
        }
        if input.get_key(KeyCode::W).pressing {
            self.transform.local_position =
                self.transform.local_position - (self.forward * mov_speed);
        }
        if input.get_key(KeyCode::Q).pressing {
            self.transform.local_position = self.transform.local_position + (up * mov_speed);
        }
        if input.get_key(KeyCode::E).pressing {
            self.transform.local_position = self.transform.local_position - (up * mov_speed);
        }

        self.update_matricies();
    }

    pub fn screen_to_world(&self, input: VecTwo) -> VecThreeFloat {
        match self.projection_type {
            ProjectionType::Perspective { focal_length } => {
                let ndc = VecThreeFloat::new(
                    ((2.0 * input.x) / self.resolution.x) - 1.0,
                    1.0 - ((2.0 * input.y) / self.resolution.y),
                    0.5,
                );

                let clip = VecFour::new(ndc.x, ndc.y, ndc.z, 0.0);

                // self.projection_mat_inverse.pretty_print();
                // println!("---");

                let view_space = M44::apply_vec_four(&self.projection_mat_inverse, &clip);
                let view_space = VecThreeFloat::new(
                    view_space.x / view_space.w,
                    view_space.y / view_space.w,
                    view_space.z / view_space.w,
                );
                let world_space = M44::apply_vec_three(&self.view_mat_inverse, &view_space);

                return world_space;

                /*
                let mut dir = world_space - self.transform.local_position;
                dir.normalize();

                let len = Self::plane_intersection_distance(
                    self.transform.local_position,
                    dir,
                    VecThreeFloat::new(0.0, 0.0, 0.0),
                    VecThreeFloat::new(0.0, -1.0, 0.0),
                )
                .unwrap();
                */

                // Check both sides of the plane
                /*
                if len < 1.0 {
                    panic!("invalid");

                    len = Self::ray_v_plane(
                        self.transform.local_position,
                        dir * -1.0,
                        VecThreeFloat::new(0.0, 0.0, 0.0),
                        VecThreeFloat::new(-1.0, 0.0, 0.0),
                    ) * -1.0;
                }
                */

                // println!("{}", len);

                // return self.transform.local_position + (dir * len);
            }
            ProjectionType::Orthographic => {
                panic!("Orthogrphic projection not implemented");
            }
        }
    }

    pub fn world_to_screen(&self, input: VecTwo) -> VecTwo {
        match self.projection_type {
            ProjectionType::Perspective { focal_length } => {
                todo!("Perspective projection not implemented here.");
            }
            ProjectionType::Orthographic => {
                // NOTE this is basically just wrong. but works becuse our world space is screen space.
                // If the projection matrix width/height doen't match the screen then this won't work.
                return input
                    - VecTwo::new(
                        self.transform.local_position.x,
                        self.transform.local_position.y,
                    );
            }
        }
    }

    /// Ray cast v plane intersection. Returns distance of ray
    fn plane_intersection_distance(
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
}

mod test {
    use super::*;

    #[test]
    pub fn plane_intersection_distance() {
        let d = Camera::plane_intersection_distance(
            VecThreeFloat::new(0.0, 0.0, 10.0),
            VecThreeFloat::new(0.0, 0.0, -1.0),
            VecThreeFloat::new(0.0, 0.0, 0.0),
            VecThreeFloat::new(0.0, 0.0, -1.0),
        );
        assert_eq!(d, Some(10.0));
    }

    #[test]
    pub fn view_matrix_identity() {
        let mut cam = Camera::new(
            ProjectionType::Perspective { focal_length: 0.9 },
            VecTwo::new(1024.0, 512.0),
        );

        cam.update_matricies();

        let mul = M44::multiply(&cam.view_mat, &cam.view_mat_inverse);
        assert!(M44::close_enough(&M44::new_identity(), &mul));
    }

    #[test]
    pub fn view_matrix_position() {
        let mut cam = Camera::new(
            ProjectionType::Perspective { focal_length: 0.9 },
            VecTwo::new(1024.0, 512.0),
        );

        cam.update_matricies();

        let point = VecThreeFloat::new(10.0, 20.5, -123.8);

        let mul = M44::multiply(&cam.view_mat, &cam.view_mat_inverse);

        let point_screen = M44::apply_vec_three(&cam.view_mat, &point);
        let point_screen_inv = M44::apply_vec_three(&cam.view_mat_inverse, &point_screen);

        assert!(VecThreeFloat::close_enough(&point, &point_screen_inv));
    }

    #[test]
    pub fn projection_matrix() {
        let mut cam = Camera::new(
            ProjectionType::Perspective { focal_length: 0.9 },
            VecTwo::new(1024.0, 512.0),
        );

        cam.update_matricies();

        let mul = M44::multiply(&cam.projection_mat, &cam.projection_mat_inverse);

        mul.pretty_print();

        assert!(M44::close_enough(&M44::new_identity(), &mul));

        /*
        let point = VecThreeFloat::new(10.0, 20.5, -123.8);
        let point_screen = M44::apply_vec_three(&cam.projection_mat, &point);
        let point_screen_inv = M44::apply_vec_three(&cam.projection_mat_inverse, &point_screen);

        assert!(VecThreeFloat::close_enough(&point, &point_screen_inv));
        */
    }
}
