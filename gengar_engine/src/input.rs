use crate::vectors::*;
use std::collections::HashMap;

pub mod key_code;
pub mod keyboard;
pub mod mouse;

pub use key_code::*;
pub use keyboard::*;
pub use mouse::*;

/// All game input
pub struct Input {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub paste: Option<String>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            paste: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ButtonState {
    pub pressing: bool,
    pub on_press: bool,
    pub on_release: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        ButtonState {
            pressing: false,
            on_press: false,
            on_release: false,
        }
    }

    pub fn update(&mut self, new_state: bool) {
        if new_state {
            self.on_release = false;

            if !self.pressing {
                self.on_press = true;
            } else {
                self.on_press = false;
            }
        } else {
            self.on_press = false;

            if self.pressing {
                self.on_release = true;
            } else {
                self.on_release = false;
            }
        }

        self.pressing = new_state;
    }
}

mod test {
    use super::*;

    #[test]
    fn press_update() {
        let mut button = ButtonState::new();

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, true);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, true);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, true);
        assert_eq!(button.on_release, false);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, true);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);
    }
}
