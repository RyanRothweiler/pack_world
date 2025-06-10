use crate::render::{render_command::*, render_pack::*};
use std::collections::HashMap;

pub struct RenderSystem {
    pub render_packs: HashMap<RenderPackID, RenderPack>,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            render_packs: HashMap::new(),
        }
    }

    pub fn get_pack(&mut self, id: RenderPackID) -> &mut RenderPack {
        self.render_packs.get_mut(&id).unwrap()
    }

    pub fn add_command(&mut self, command: RenderCommand, pack_id: RenderPackID) {
        self.get_pack(RenderPackID::Shop).commands.push(command);
    }
}
