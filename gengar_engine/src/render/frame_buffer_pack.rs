pub struct FrameBufferPack {
    pub frame_buffer: u32,
    pub color_buffer: u32,
}

impl FrameBufferPack {
    pub fn new() -> Self {
        Self {
            frame_buffer: 0,
            color_buffer: 0,
        }
    }
}
