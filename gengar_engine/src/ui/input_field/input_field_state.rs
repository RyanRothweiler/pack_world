use crate::{input::*, rect::*, ui::*, vectors::*};

pub struct InputFieldData {
    pub state: crate::ui::button::ButtonState,
    pub selected: bool,

    pub blink_accum: f64,
}

impl InputFieldData {
    pub fn new() -> Self {
        Self {
            state: crate::ui::button::ButtonState::Idle,
            selected: false,
            blink_accum: 0.0,
        }
    }

    pub fn update(&mut self, hitbox: Rect, mouse: &Mouse) {
        // self.on_down = mouse.button_left.on_press;

        if hitbox.contains(mouse.pos) {
            if mouse.button_left.pressing {
                self.state = crate::ui::button::ButtonState::Down;
                self.selected = true;
            } else {
                self.state = crate::ui::button::ButtonState::Hovering;
            }
        } else {
            self.state = crate::ui::button::ButtonState::Idle;
        }
    }
}
