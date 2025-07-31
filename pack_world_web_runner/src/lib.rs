#![allow(static_mut_refs, clippy::all)]

use elara_engine::game_methods::*;
use elara_render_opengl::OglRenderApi;
use pack_world_game::state::*;
use std::ffi::c_void;
use wasm_bindgen::prelude::*;
use web_sys::{BeforeUnloadEvent, ClipboardEvent, KeyboardEvent, MouseEvent, WheelEvent};

static mut GAME_STATE: Option<State> = None;
static mut GAME_METHODS: Option<GameMethods<OglRenderApi>> = None;

#[wasm_bindgen(start)]
pub fn start() {
    unsafe {
        GAME_STATE = Some(State::new());
        GAME_METHODS = Some(GameMethods::<OglRenderApi> {
            init: pack_world_game::game_init,
            update: pack_world_game::game_loop,
        });

        let state_ref = GAME_STATE.as_mut().unwrap();
        let methods_ref = GAME_METHODS.as_ref().unwrap();

        let state_ptr = state_ref as *mut _ as *mut c_void;
        elara_platform_web::start(state_ptr, methods_ref);
    }
}

#[wasm_bindgen]
pub fn main_loop() {
    unsafe {
        let state_ref = GAME_STATE.as_mut().unwrap();
        let methods_ref = GAME_METHODS.as_ref().unwrap();

        let state_ptr = state_ref as *mut _ as *mut c_void;
        elara_platform_web::update(state_ptr, methods_ref);
    }
}

#[wasm_bindgen]
pub fn key_down(event: KeyboardEvent) {
    elara_platform_web::key_down(event);
}

#[wasm_bindgen]
pub fn key_up(event: KeyboardEvent) {
    elara_platform_web::key_up(event);
}

#[wasm_bindgen]
pub fn mouse_down(event: MouseEvent) {
    elara_platform_web::mouse_down(event);
}

#[wasm_bindgen]
pub fn mouse_up(event: MouseEvent) {
    elara_platform_web::mouse_up(event);
}

#[wasm_bindgen]
pub fn mouse_move(event: MouseEvent) {
    elara_platform_web::mouse_move(event);
}

#[wasm_bindgen]
pub fn on_before_unload(event: BeforeUnloadEvent) {
    elara_platform_web::on_before_unload(event);
}

#[wasm_bindgen]
pub fn paste_handler(event: ClipboardEvent) {
    elara_platform_web::paste_handler(event);
}

#[wasm_bindgen]
pub fn mouse_wheel_handler(event: WheelEvent) {
    elara_platform_web::mouse_wheel_handler(event);
}
