#![allow(
    unused_variables,
    unused_imports,
    dead_code,
    unused_assignments,
    static_mut_refs,
    clippy::all,
    unreachable_code
)]

use game::{game_init, game_loop, state::*};
use gengar_engine::{input::*, state::State as EngineState, vectors::*};

use wasm_bindgen::prelude::*;
use web_sys::{
    console, KeyboardEvent, MouseEvent, WebGl2RenderingContext, WebGlProgram, WebGlShader,
};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod webgl;

use webgl::{webgl_render::*, webgl_render_api::*};

static mut ENGINE_STATE: Option<EngineState> = None;
static mut GAME_STATE: Option<game::state::State> = None;
static mut RENDER_API: Option<WebGLRenderApi> = None;
static mut INPUT: Option<Input> = None;

static mut MOUSE_POS: VecTwo = VecTwo { x: 0.0, y: 0.0 };
static mut MOUSE_LEFT_DOWN: bool = false;
static mut MOUSE_RIGHT_DOWN: bool = false;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn log(input: &str) {
    console::log_1(&input.into());
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let gl_state = webgl::webgl_render_api::WebGLState {
        programs: HashMap::new(),
        next_prog_id: 0,

        vaos: HashMap::new(),
        next_vao_id: 0,

        textures: HashMap::new(),
        next_texture_id: 0,
    };

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("gengar_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let resolution = VecTwo::new(canvas.client_width() as f64, canvas.client_height() as f64);

    let gl_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    unsafe {
        webgl::webgl_render_api::GL_STATE = Some(gl_state);
        webgl::webgl_render_api::GL_CONTEXT = Some(gl_context);

        RENDER_API = Some(get_render_api());
        INPUT = Some(Input::new());

        ENGINE_STATE = Some(gengar_engine::state::State::new(resolution));
        GAME_STATE = Some(game::state::State::new());

        todo!();

        /*
        gengar_engine::load_resources(
            &mut ENGINE_STATE.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
        );

        game_init(
            GAME_STATE.as_mut().unwrap(),
            ENGINE_STATE.as_mut().unwrap(),
            // new_engine_state,
            RENDER_API.as_mut().unwrap(),
        );
        */
    };
}

#[wasm_bindgen]
pub fn key_down(vent: KeyboardEvent) {
    let input: &mut Input = unsafe { INPUT.as_mut().unwrap() };
    // input.keyboard[vent.key_code() as usize].update(true);
    todo!("fix keyboard input")
}

#[wasm_bindgen]
pub fn mouse_down(vent: MouseEvent) {
    unsafe {
        if vent.button() == 0 {
            MOUSE_LEFT_DOWN = true;
        } else if vent.button() == 2 {
            MOUSE_RIGHT_DOWN = true;
        }
    }
}

#[wasm_bindgen]
pub fn mouse_up(vent: MouseEvent) {
    unsafe {
        if vent.button() == 0 {
            MOUSE_LEFT_DOWN = false;
        } else if vent.button() == 2 {
            MOUSE_RIGHT_DOWN = false;
        }
    }
}

#[wasm_bindgen]
pub fn key_up(vent: KeyboardEvent) {
    let input: &mut Input = unsafe { INPUT.as_mut().unwrap() };
    // input.keyboard[vent.key_code() as usize].update(false);
    todo!("fix keyboard input")
}

#[wasm_bindgen]
pub fn mouse_move(vent: MouseEvent) {
    unsafe {
        MOUSE_POS.x = vent.client_x() as f64;
        MOUSE_POS.y = vent.client_y() as f64;
    };
}

#[wasm_bindgen]
pub fn main_loop() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("gengar_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let resolution = VecTwo::new(canvas.client_width() as f64, canvas.client_height() as f64);

    let gl_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    unsafe {
        todo!("Update to handle new allocation syste");

        // Update input
        {
            let input: &mut Input = INPUT.as_mut().unwrap();

            input.mouse.button_left.update(MOUSE_LEFT_DOWN);
            input.mouse.button_right.update(MOUSE_RIGHT_DOWN);

            // Mouse position
            let prev_pos = input.mouse.pos;
            input.mouse.pos = MOUSE_POS;
            input.mouse.pos_delta = VecTwo::new(
                prev_pos.x - input.mouse.pos.x as f64,
                prev_pos.y - input.mouse.pos.y as f64,
            );
        }

        /*
        gengar_engine::engine_frame_start(
            ENGINE_STATE.as_mut().unwrap(),
            INPUT.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
        );
        game_loop(
            0.001,
            GAME_STATE.as_mut().unwrap(),
            ENGINE_STATE.as_mut().unwrap(),
            INPUT.as_mut().unwrap(),
        );
        gengar_engine::engine_frame_end(ENGINE_STATE.as_mut().unwrap());
        */

        let light_trans = ENGINE_STATE.as_mut().unwrap().transforms
            [GAME_STATE.as_mut().unwrap().light_trans.unwrap()]
        .global_matrix
        .get_position();

        render(
            ENGINE_STATE.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
            &resolution,
            &gl_context,
            light_trans,
        );
    }
}
