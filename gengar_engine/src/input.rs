use crate::vectors::*;
use std::collections::HashMap;

pub mod mouse;

pub use mouse::*;

pub struct Input {
    pub mouse: Mouse,
    pub keyboard: HashMap<KeyCode, ButtonState>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse: Mouse::new(),
            keyboard: HashMap::new(),
        }
    }

    pub fn key_pressed(&self) -> Option<KeyCode> {
        for (key, value) in &self.keyboard {
            if value.on_press {
                return Some(*key);
            }
        }

        return None;
    }

    pub fn get_key(&self, key_code: KeyCode) -> ButtonState {
        // this returns an emtpy button state, if the button is missing from the hashmap.
        // once the button is pressed it'll be added to the hashmap
        // this avoids needing to pass a mutable self here
        if !self.keyboard.contains_key(&key_code) {
            return ButtonState::new();
        }

        return *self.keyboard.get(&key_code).unwrap();
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,

    Spacebar,
    Tab,
    Escape,
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
