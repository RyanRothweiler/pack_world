#![allow(unused_variables)]

use elara_engine::{input::*, platform_api::*, state::State as EngineState};
use elara_render_opengl::*;
use std::ffi::c_void;

pub struct GameState {
    pub state_here: i32,
}

#[no_mangle]
pub fn game_init(
    game_state_ptr: *mut c_void,
    es: &mut EngineState,
    render_api: &mut OglRenderApi,
    platform_api: &PlatformApi,
) {
    let gs = unsafe { &mut *(game_state_ptr as *mut GameState) };

    println!("game init");
}

#[no_mangle]
fn game_loop(
    prev_delta_time: f64,
    game_state_ptr: *mut c_void,
    es: &mut EngineState,
    input: &mut Input,
    render_api: &mut OglRenderApi,
    platform_api: &PlatformApi,
) {
    let gs = unsafe { &mut *(game_state_ptr as *mut GameState) };

    println!("game state here {}", gs.state_here);
    gs.state_here += 1;
}
