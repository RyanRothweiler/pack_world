use crate::vectors::*;

// Used for any FFI C stuff. Like ogl graphics rendering
#[repr(C)]
pub struct VecTwoC {
    pub x: f32,
    pub y: f32,
}

impl From<&VecTwo> for VecTwoC {
    fn from(input: &VecTwo) -> Self {
        Self {
            x: input.x as f32,
            y: input.y as f32,
        }
    }
}
