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
use gengar_engine::{input::*, platform_api::PlatformApi, state::State as EngineState, vectors::*};
use js_sys::Math;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{LazyLock, Mutex},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    console, KeyboardEvent, MouseEvent, Request, RequestInit, RequestMode, Response,
    WebGl2RenderingContext, WebGlProgram, WebGlShader,
};

mod webgl;

use webgl::{webgl_render::*, webgl_render_api::*};

static mut ENGINE_STATE: Option<EngineState> = None;
static mut GAME_STATE: Option<game::state::State> = None;
static mut RENDER_API: Option<WebGLRenderApi> = None;
static mut INPUT: Option<Input> = None;

static KEYBOARD: LazyLock<Mutex<HashMap<KeyCode, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

fn rand() -> f64 {
    Math::random()
}

async fn send_event() {
    log("sending event");

    let event_id = "app_start_testing";

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);

    opts.set_body(&wasm_bindgen::JsValue::from_str("[1, 2, 3]"));

    let url = "https://api.mixpanel.com/track";

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    request
        .headers()
        .set("content-type", "application/json")
        .unwrap();
    request.headers().set("accept", "text/plain").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    log(&format!("finished sending {}", resp.status()));
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

        buffers: HashMap::new(),
        next_buffer_id: 0,
    };

    let window = web_sys::window().unwrap();
    let performance = window
        .performance()
        .expect("performance should be available");
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("gengar_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let resolution = VecTwo::new(canvas.client_width() as f64, canvas.client_height() as f64);

    let context_attributes = web_sys::WebGlContextAttributes::new();
    context_attributes.set_alpha(false);
    context_attributes.set_antialias(true);
    context_attributes.set_premultiplied_alpha(false);

    // wasm_bindgen_futures::spawn_local(send_event());

    let gl_context = canvas
        .get_context_with_context_options("webgl2", &context_attributes)
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

        gengar_engine::load_resources(
            &mut ENGINE_STATE.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
        );

        game_init(
            GAME_STATE.as_mut().unwrap(),
            ENGINE_STATE.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
        );

        PREV_TIME = performance.now();
    };
}

#[wasm_bindgen]
pub fn key_down(vent: KeyboardEvent) {
    if let Some(key) = to_keycode(vent.key()) {
        KEYBOARD.lock().unwrap().insert(key, true);
    }
}

#[wasm_bindgen]
pub fn key_up(vent: KeyboardEvent) {
    if let Some(key) = to_keycode(vent.key()) {
        KEYBOARD.lock().unwrap().insert(key, false);
    }
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
pub fn mouse_move(vent: MouseEvent) {
    unsafe {
        MOUSE_POS.x = vent.client_x() as f64;
        MOUSE_POS.y = vent.client_y() as f64;
    };
}

static mut PREV_TIME: f64 = 0.0;

#[wasm_bindgen]
pub fn main_loop() {
    let platform_api = PlatformApi { rand: rand };

    let window = web_sys::window().unwrap();
    let performance = window
        .performance()
        .expect("performance should be available");
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("gengar_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let resolution = VecTwo::new(canvas.client_width() as f64, canvas.client_height() as f64);

    let context_attributes = web_sys::WebGlContextAttributes::new();
    context_attributes.set_alpha(false);
    context_attributes.set_antialias(true);
    context_attributes.set_premultiplied_alpha(false);

    let gl_context = canvas
        .get_context_with_context_options("webgl2", &context_attributes)
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    unsafe {
        let time_start = performance.now();
        let prev_frame_dur = performance.now() - PREV_TIME;
        PREV_TIME = time_start;

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

            let key_states: &HashMap<KeyCode, bool> = &KEYBOARD.lock().unwrap();
            for (key, value) in key_states {
                input
                    .keyboard
                    .entry(*key)
                    .or_insert(ButtonState::new())
                    .update(*value);
            }
        }

        gengar_engine::engine_frame_start(
            ENGINE_STATE.as_mut().unwrap(),
            INPUT.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
        );
        game_loop(
            prev_frame_dur / 1000.0,
            GAME_STATE.as_mut().unwrap(),
            ENGINE_STATE.as_mut().unwrap(),
            INPUT.as_mut().unwrap(),
            &platform_api,
        );
        gengar_engine::engine_frame_end(ENGINE_STATE.as_mut().unwrap());

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

pub fn to_keycode(key: String) -> Option<KeyCode> {
    let st: &str = &key;
    match st {
        "a" => Some(KeyCode::A),
        "b" => Some(KeyCode::B),
        "c" => Some(KeyCode::C),
        "d" => Some(KeyCode::D),
        "e" => Some(KeyCode::E),
        "f" => Some(KeyCode::F),
        "g" => Some(KeyCode::G),
        "h" => Some(KeyCode::H),
        "i" => Some(KeyCode::I),
        "j" => Some(KeyCode::J),
        "k" => Some(KeyCode::K),
        "l" => Some(KeyCode::L),
        "m" => Some(KeyCode::M),
        "n" => Some(KeyCode::N),
        "o" => Some(KeyCode::O),
        "p" => Some(KeyCode::P),
        "q" => Some(KeyCode::Q),
        "r" => Some(KeyCode::R),
        "s" => Some(KeyCode::S),
        "t" => Some(KeyCode::T),
        "u" => Some(KeyCode::U),
        "v" => Some(KeyCode::V),
        "w" => Some(KeyCode::W),
        "x" => Some(KeyCode::X),
        "y" => Some(KeyCode::Y),
        "z" => Some(KeyCode::Z),

        "0" => Some(KeyCode::Zero),
        "1" => Some(KeyCode::One),
        "2" => Some(KeyCode::Two),
        "3" => Some(KeyCode::Three),
        "4" => Some(KeyCode::Four),
        "5" => Some(KeyCode::Five),
        "6" => Some(KeyCode::Six),
        "7" => Some(KeyCode::Seven),
        "8" => Some(KeyCode::Eight),
        "9" => Some(KeyCode::Nine),

        "Tab" => Some(KeyCode::Tab),
        "Escape" => Some(KeyCode::Escape),
        " " => Some(KeyCode::Spacebar),
        _ => {
            log(&format!("Unknown keycode {:?}", key));
            return None;
        }
    }
}
