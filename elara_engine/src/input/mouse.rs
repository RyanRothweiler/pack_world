use crate::{input::ButtonState, vectors::*};

#[derive(Clone)]
pub struct Mouse {
    pub pos: VecTwo,
    pub pos_delta: VecTwo,
    pub button_left: ButtonState,
    pub button_right: ButtonState,

    pub scroll_delta: i32,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            pos: VecTwo::new(0.0, 0.0),
            pos_delta: VecTwo::new(0.0, 0.0),
            button_left: ButtonState::new(),
            button_right: ButtonState::new(),

            scroll_delta: 0,
        }
    }
}
