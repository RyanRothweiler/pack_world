use crate::vectors::*;

// Used for any FFI C stuff. Like ogl graphics rendering
#[repr(C)]
pub struct VecThreeFloatC {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&VecThreeFloat> for VecThreeFloatC {
    fn from(input: &VecThreeFloat) -> Self {
        VecThreeFloatC {
            x: input.x as f32,
            y: input.y as f32,
            z: input.z as f32,
        }
    }
}
