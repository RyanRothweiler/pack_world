use crate::{
    render::{camera::*, render_command::*},
    vectors::*,
};

#[derive(Eq, PartialEq, Hash)]
pub enum RenderPackID {
    UI,
    World,
}

pub struct RenderPack {
    pub commands: Vec<RenderCommand>,
    pub camera: Camera,
}

impl RenderPack {
    pub fn new(projection_type: ProjectionType, window_resolution: VecTwo) -> Self {
        Self {
            commands: vec![],
            camera: Camera::new(projection_type, window_resolution),
        }
    }
}
