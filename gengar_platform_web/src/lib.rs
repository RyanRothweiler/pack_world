#![allow(
    unused_variables,
    // unused_imports,
    dead_code,
    unused_assignments,
    static_mut_refs,
    clippy::all,
    unreachable_code
)]

use game::{game_init, game_loop};
use gengar_engine::{
    analytics::*, error::Error, input::*, platform_api::PlatformApi, state::State as EngineState,
    vectors::*,
};
use js_sys::Math;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    console, Headers, KeyboardEvent, MouseEvent, Request, RequestInit, Response,
    WebGl2RenderingContext,
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

static USER_ID: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".into()));
const USER_ID_KEY: &str = "USER_ID_KEY";

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

fn send_event(event: AnalyticsEvent) {
    wasm_bindgen_futures::spawn_local(send_event_async(event));
}

// supabase storage api info
// https://stackoverflow.com/questions/75540112/how-to-upload-to-supabase-storage-using-curl
async fn upload_data(data: Vec<u8>) {
    let opts = RequestInit::new();
    opts.set_method("POST");

    opts.set_body(&JsValue::from_str(unsafe {
        std::str::from_utf8_unchecked(&data)
    }));

    let headers = Headers::new().unwrap();
    headers.set("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWJxamxndmtoenlyamFhYnZnIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDIzMTc1MTUsImV4cCI6MjA1Nzg5MzUxNX0.wYCDHY5jXVIex2E6ZmzU16DQC5GtqMiPV974N7TQKUM").unwrap();
    headers
    .set("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWJxamxndmtoenlyamFhYnZnIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0MjMxNzUxNSwiZXhwIjoyMDU3ODkzNTE1fQ.uNXhoOMoAKyjcN2A2Iss1AIwCns46V9abIaGC_luQBk")
    .unwrap();
    headers.set("x-upsert", "true").unwrap();

    opts.set_headers(&headers);

    let url = format!(
        "https://qqibqjlgvkhzyrjaabvg.supabase.co/storage/v1/object/saves-public/{}.gsf",
        USER_ID.lock().unwrap(),
    );

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    log("Save upload successful");
}

async fn send_event_async(event: AnalyticsEvent) {
    log(&format!("Analytics Sending {:?}", event));

    let data = format!("{{ \"type\": \"track\", \"payload\": {{ \"name\": \"{}\", \"properties\": {{ \"test\": \"property\" }} }} }}", event.to_id());

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&wasm_bindgen::JsValue::from_str(&data));

    let headers = Headers::new().unwrap();
    headers.set("content-type", "application/json").unwrap();
    headers
        .set("openpanel-client-secret", "sec_6a215d5e6eb414d83b73")
        .unwrap();
    headers
        .set(
            "openpanel-client-id",
            "6664df8c-1cf1-410d-8812-e4c06aca2b1c",
        )
        .unwrap();

    opts.set_headers(&headers);

    let request = Request::new_with_str_and_init("https://api.openpanel.dev/track", &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    log(&format!("Sent {:?} -> {}", event, resp.status()));
}

fn write_save_game_data(data: Vec<u8>) -> Result<(), Error> {
    // call upload data to write to supabase
    todo!("save_game")
}

fn get_save_game_data() -> Result<Vec<u8>, Error> {
    // download the data from supabase
    todo!("load_game");
    let ret: Vec<u8> = vec![];
    Ok(ret)
}

pub fn get_platform_api() -> PlatformApi {
    PlatformApi {
        rand: rand,
        send_event: send_event,
        write_save_game_data: write_save_game_data,
        get_save_game_data: get_save_game_data,
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();

    // get / setup userid
    {
        let ls = window.local_storage().unwrap().unwrap();
        let user_id: String = match ls.get_item(USER_ID_KEY).unwrap() {
            Some(key) => key,
            None => {
                let uuid: String = window.crypto().unwrap().random_uuid();
                ls.set_item(USER_ID_KEY, &uuid).unwrap();
                log("Generated new user_id");

                uuid
            }
        };

        log(&format!("user_id {}", user_id));
        *USER_ID.lock().unwrap() = user_id;
    }

    let platform_api = get_platform_api();
    console_error_panic_hook::set_once();

    // wasm_bindgen_futures::spawn_local(upload_test(vec![10, 9, 10, 9]));

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
            &platform_api,
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
    let platform_api = get_platform_api();

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
