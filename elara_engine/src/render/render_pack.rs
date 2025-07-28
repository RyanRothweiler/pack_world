use crate::{
    render::{camera::*, light::*, render_command::*},
    vectors::*,
};

#[derive(Eq, PartialEq, Hash)]
pub enum RenderPackID {
    UI,
    World,

    // 3d camera
    NewWorld,

    // 3d camera. This is game specific that probably shouldn't be here
    Shop,
}

pub struct RenderPack {
    pub commands: Vec<RenderCommand>,
    pub camera: Camera,

    // only support one light for now
    pub lights: Vec<Light>,
}

impl RenderPack {
    pub fn new(projection_type: ProjectionType, window_resolution: VecTwo) -> Self {
        Self {
            commands: vec![],
            lights: vec![],
            camera: Camera::new(projection_type, window_resolution),
        }
    }
}
