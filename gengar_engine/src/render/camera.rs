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
            far_plane: 1000.0,
            fov: 0.0,

            yaw: 90.0,
            pitch: 0.0,
        }
    }

    pub fn update_matricies(&mut self) {
        match &self.projection_type {
            ProjectionType::Perspective { focal_length } => {
                let aspect = self.resolution.x / self.resolution.y;
                let fov_y = 45.0_f64.to_radians();
                let f = 1.0 / (fov_y / 2.0).tan();

                let a = f / aspect;
                let b = f;
                let c = (self.far_plane + self.near_plane) / (self.near_plane - self.far_plane);
                let d =
                    (2.0 * self.far_plane * self.near_plane) / (self.near_plane - self.far_plane);

                self.projection_mat = M44::new_empty();
                self.projection_mat.set(0, 0, a);
                self.projection_mat.set(1, 1, b);
                self.projection_mat.set(2, 2, c);
                self.projection_mat.set(2, 3, -1.0);
                self.projection_mat.set(3, 2, d);
                self.projection_mat.set(3, 3, 0.0);

                self.projection_mat_inverse = M44::new_empty();
                self.projection_mat_inverse.set(0, 0, 1.0 / a);
                self.projection_mat_inverse.set(1, 1, 1.0 / b);
                self.projection_mat_inverse.set(2, 3, 1.0 / d);
                self.projection_mat_inverse.set(3, 2, -1.0);
                self.projection_mat_inverse.set(3, 3, c / d);
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
            let eye = self.transform.local_position;

            let up = VecThreeFloat::new(0.0, 1.0, 0.0);

            // Cam yaw / pitch axis
            self.forward = VecThreeFloat::new_zero();
            self.forward.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
            self.forward.y = self.pitch.to_radians().sin();
            self.forward.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
            self.forward.normalize_self();

            let cam_dir = self.transform.local_position + (self.forward * -1.0);

            let cam_dir = (eye - cam_dir).normalize();
            let cam_right = VecThreeFloat::cross(up, cam_dir).normalize();
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
                self.view_mat_inverse.set(0, 1, ix.y);
                self.view_mat_inverse.set(0, 2, ix.z);

                self.view_mat_inverse.set(1, 0, iy.x);
                self.view_mat_inverse.set(1, 1, iy.y);
                self.view_mat_inverse.set(1, 2, iy.z);

                self.view_mat_inverse.set(2, 0, iz.x);
                self.view_mat_inverse.set(2, 1, iz.y);
                self.view_mat_inverse.set(2, 2, iz.z);

                self.view_mat_inverse.set(3, 0, eye.x);
                self.view_mat_inverse.set(3, 1, eye.y);
                self.view_mat_inverse.set(3, 2, eye.z);
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
        right.normalize_self();

        let mut up = VecThreeFloat::cross(self.forward, right);
        up.normalize_self();

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
                let x = (2.0 * input.x) / self.resolution.x - 1.0;
                let y = 1.0 - (2.0 * input.y) / self.resolution.y;

                let z_near = -1.0; // or -1.0 if OpenGL-style
                let z_far = 1.0;

                let clip_near = VecFour::new(x, y, z_near, 1.0);
                let clip_far = VecFour::new(x, y, z_far, 1.0);

                let view_near = M44::apply_vec_four(&self.projection_mat_inverse, &clip_near);
                let view_near = VecThreeFloat::new(
                    view_near.x / view_near.w,
                    view_near.y / view_near.w,
                    view_near.z / view_near.w,
                );

                let view_far = M44::apply_vec_four(&self.projection_mat_inverse, &clip_far);
                let view_far = VecThreeFloat::new(
                    view_far.x / view_far.w,
                    view_far.y / view_far.w,
                    view_far.z / view_far.w,
                );

                let world_near: VecThreeFloat =
                    M44::apply_vec_three(&self.view_mat_inverse, &view_near);
                let world_far: VecThreeFloat =
                    M44::apply_vec_three(&self.view_mat_inverse, &view_far);

                let mut dir = world_far - world_near;
                dir.normalize_self();

                // let pos = self.transform.local_position + (dir * 10.0);
                let pos = world_near + (dir * 10.0);
                return pos;
                // (world_near, dir)
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
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_camera() -> Camera {
        let mut cam = Camera::new(
            ProjectionType::Perspective { focal_length: 0.9 },
            VecTwo::new(1024.0, 512.0),
        );

        cam.transform.local_position = VecThreeFloat::new(10.0, 1.0, 5.0);
        cam.yaw = 20.0;
        cam.pitch = 100.0;
        cam.update_matricies();

        return cam;
    }

    #[test]
    pub fn view_matrix_identity() {
        let cam = get_test_camera();

        let mul = M44::multiply(&cam.view_mat, &cam.view_mat_inverse);

        mul.pretty_print();

        assert!(M44::close_enough(&M44::new_identity(), &mul));
    }

    // #[test]
    pub fn view_matrix_position() {
        let cam = get_test_camera();

        let point = VecThreeFloat::new(10.0, 20.5, -123.8);

        let mul = M44::multiply(&cam.view_mat, &cam.view_mat_inverse);

        let point_screen = M44::apply_vec_three(&cam.view_mat, &point);
        let point_screen_inv = M44::apply_vec_three(&cam.view_mat_inverse, &point_screen);

        assert!(VecThreeFloat::close_enough(&point, &point_screen_inv));
    }

    #[test]
    pub fn projection_matrix() {
        let cam = get_test_camera();

        let mul = M44::multiply(&cam.projection_mat, &cam.projection_mat_inverse);

        mul.pretty_print();

        assert!(M44::close_enough(&M44::new_identity(), &mul));
    }
}
