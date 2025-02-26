use crate::{input::*, rect::*, vectors::*};

#[derive(Eq, PartialEq)]
pub enum ButtonState {
    Hovering,
    Down,
    Idle,
}

pub struct ButtonData {
    pub state: ButtonState,
    pub on_down: bool,

    pub image_shrink: f64,
}

impl ButtonData {
    pub fn new() -> Self {
        Self {
            state: ButtonState::Idle,
            image_shrink: 0.0,
            on_down: false,
        }
    }

    pub fn update(&mut self, hitbox: Rect, mouse: &Mouse, delta_time: f64) {
        self.on_down = mouse.button_left.on_press;

        if hitbox.contains(mouse.pos) {
            if mouse.button_left.pressing {
                self.state = ButtonState::Down;
            } else {
                self.state = ButtonState::Hovering;
            }
        } else {
            self.state = ButtonState::Idle;
        }
    }
}
