use crate::{ButtonState, KeyCode};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Keyboard {
    pub keys: HashMap<KeyCode, ButtonState>,
    pub char_down: Option<char>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            char_down: None,
        }
    }

    pub fn key_pressed(&self) -> Option<KeyCode> {
        for (key, value) in &self.keys {
            if value.on_press {
                return Some(*key);
            }
        }

        return None;
    }

    pub fn get_key(&self, key_code: KeyCode) -> ButtonState {
        // this returns an emtpy button state, if the button is missing from the hashmap.
        // once the button is pressed it'll be added to the hashmap by the caller
        // this avoids needing to pass a mutable self here
        if !self.keys.contains_key(&key_code) {
            return ButtonState::new();
        }

        return *self.keys.get(&key_code).unwrap();
    }
}
