#![allow(
    unused_variables,
    dead_code,
    unused_assignments,
    // unused_imports,
    // unreachable_code,
    static_mut_refs,
    clippy::all
)]

// windows hello triangle in rust
// https://rust-tutorials.github.io/triangle-from-scratch/loading_opengl/win32.html

// example of entire setup
// https://github.com/glowcoil/raw-gl-context/blob/master/src/win.rs

mod gl;
mod vol_mem;

use game;
use gengar_engine::{byte_conversion::*, error::Error as EngineError, input::*, vectors::*};
use gengar_render_opengl::*;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
    thread,
    time::{Duration, SystemTime},
};
use vol_mem::*;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::{Gdi::*, OpenGL::*},
        Storage::FileSystem::*,
        System::LibraryLoader::*,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

#[global_allocator]
static A: TrackingAlloc = TrackingAlloc;

const FRAME_TARGET_FPS: f64 = 60.0;
const FRAME_TARGET: Duration = Duration::from_secs((1.0 / FRAME_TARGET_FPS) as u64);

static mut GAME_DLL_PATH: PCWSTR = w!("");
static mut GAME_DLL_CURRENT_PATH: PCWSTR = w!("");

type FuncWglChoosePixelFormatARB =
    extern "stdcall" fn(HDC, *const i32, *const f32, u32, *mut i32, *mut i32) -> i32;

type FuncWglCreateContextAttribsARB = extern "system" fn(HDC, i32, *const i32) -> HGLRC;

static mut RUNNING: bool = true;

static mut MOUSE_LEFT_DOWN: bool = false;
static mut MOUSE_RIGHT_DOWN: bool = false;
static mut KEYBOARD_NEW: LazyLock<Mutex<HashMap<KeyCode, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

type FuncGameInit = fn(
    &mut game::state::State,
    &mut gengar_engine::state::State,
    &gengar_render_opengl::OglRenderApi,
);
type FuncGameLoop = fn(
    f64,
    &mut game::state::State,
    &mut gengar_engine::state::State,
    &mut gengar_engine::input::Input,
);

struct GameDll {
    dll_handle: HMODULE,
    file_write_time: FILETIME,
    proc_init: FuncGameInit,
    proc_loop: FuncGameLoop,
}

fn main() {
    memory_track!("main");

    let dll_path = format!("{}.dll", game::PACKAGE_NAME);
    let dll_current_path = format!("{}_current.dll", game::PACKAGE_NAME);

    // These need to be here. Pointers are taken from them, so they cannot be dropped.
    let h_dll_path = HSTRING::from(dll_path);
    let h_dll_current_path = HSTRING::from(dll_current_path);

    unsafe {
        GAME_DLL_PATH = PCWSTR(h_dll_path.as_ptr());
        GAME_DLL_CURRENT_PATH = PCWSTR(h_dll_current_path.as_ptr());
    }

    unsafe {
        let instance = GetModuleHandleA(None).unwrap();

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: instance.into(),
            lpszClassName: s!("main_window_class"),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(windows_callback),
            ..Default::default()
        };

        let result = RegisterClassA(&wc);
        if result == 0 {
            eprintln!("Error register windows class");
            return;
        }

        let resolution = VecTwo::new(1920.0, 1080.0);

        // create main window
        let main_window_handle = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            wc.lpszClassName,
            s!("game"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            resolution.x as i32,
            resolution.y as i32,
            None,
            None,
            instance,
            None,
        );

        // functions to get
        let mut wgl_choose_pixel_format_arb: Option<FuncWglChoosePixelFormatARB> = None;
        let mut wgl_create_context_attribs: Option<FuncWglCreateContextAttribsARB> = None;

        // Use dummy device context to get the proc addresses needed for the final window
        {
            let dummy_wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance.into(),
                lpszClassName: s!("dummy_window"),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(dummy_windows_callback),
                ..Default::default()
            };

            let dummy_atom = RegisterClassA(&dummy_wc);
            debug_assert!(dummy_atom != 0);

            let dummy_win_handle = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                dummy_wc.lpszClassName,
                s!("ghostly_dummy"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                None,
            );

            let dummy_device_context = GetDC(dummy_win_handle);

            let nsize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
            let dummy_desired_pixel_format: PIXELFORMATDESCRIPTOR = PIXELFORMATDESCRIPTOR {
                nSize: nsize,
                nVersion: 1,
                iPixelType: PFD_TYPE_RGBA,
                dwFlags: PFD_SUPPORT_OPENGL | PFD_DRAW_TO_WINDOW | PFD_DOUBLEBUFFER,
                cColorBits: 32,
                cAlphaBits: 8,
                cDepthBits: 24,
                cStencilBits: 8,
                iLayerType: gl::PFD_MAIN_PLANE,
                ..Default::default()
            };

            let suggested_pixel_format_index: i32 =
                ChoosePixelFormat(dummy_device_context, &dummy_desired_pixel_format);
            if suggested_pixel_format_index == 0 {
                println!("Invalid pixel format");
            }

            SetPixelFormat(
                dummy_device_context,
                suggested_pixel_format_index,
                &dummy_desired_pixel_format,
            )
            .unwrap();

            let dummy_opengl_context = wglCreateContext(dummy_device_context).unwrap();
            wglMakeCurrent(dummy_device_context, dummy_opengl_context).unwrap();

            // get proc addresses
            let wgl_choose_pixel_format_arb_proc =
                wglGetProcAddress(s!("wglChoosePixelFormatARB")).unwrap();
            wgl_choose_pixel_format_arb =
                Some(std::mem::transmute(wgl_choose_pixel_format_arb_proc));

            let wgl_create_context_attribs_proc =
                wglGetProcAddress(s!("wglCreateContextAttribsARB")).unwrap();
            wgl_create_context_attribs = Some(std::mem::transmute(wgl_create_context_attribs_proc));

            wglDeleteContext(dummy_opengl_context).expect("error");
            wglMakeCurrent(
                dummy_device_context,
                windows::Win32::Graphics::OpenGL::HGLRC::default(),
            )
            .unwrap();
            ReleaseDC(dummy_win_handle, dummy_device_context);
            DestroyWindow(dummy_win_handle).unwrap();
        }

        // init opengl
        let device_context = GetDC(main_window_handle);

        // setup real opengl window
        #[rustfmt::skip]
        let pixel_format_attribs: [i32; 17] = [
            gl::WGL_DRAW_TO_WINDOW_ARB as i32,      1,
            gl::WGL_SUPPORT_OPENGL_ARB as i32,      1,
            gl::WGL_DOUBLE_BUFFER_ARB as i32,       1,
            gl::WGL_PIXEL_TYPE_ARB as i32,          gl::WGL_TYPE_RGBA_ARB as i32,
            gl::WGL_ACCELERATION_ARB as i32,        gl::WGL_FULL_ACCELERATION_ARB as i32,

            gl::WGL_COLOR_BITS_ARB as i32,          32,
            gl::WGL_DEPTH_BITS_ARB as i32,          24,
            gl::WGL_STENCIL_BITS_ARB as i32,        8,

            0,
        ];
        let mut extend_pick: i32 = 0;
        let mut suggested_pixel_format_index: i32 = 0;
        let res = (wgl_choose_pixel_format_arb.unwrap())(
            device_context,
            pixel_format_attribs.as_ptr(),
            std::ptr::null(),
            1,
            &mut suggested_pixel_format_index,
            &mut extend_pick,
        );

        let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
        DescribePixelFormat(
            device_context,
            suggested_pixel_format_index,
            std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32,
            Some(&mut pfd),
        );
        SetPixelFormat(device_context, suggested_pixel_format_index, &pfd).unwrap();

        #[rustfmt::skip]
        let context_attribs = [
            gl::WGL_CONTEXT_MAJOR_VERSION_ARB as i32, 3 as i32,
            gl::WGL_CONTEXT_MINOR_VERSION_ARB as i32, 3 as i32,
            gl::WGL_CONTEXT_PROFILE_MASK_ARB as i32, gl::WGL_CONTEXT_CORE_PROFILE_BIT_ARB as i32,
            0
        ];

        let wgl_context =
            wgl_create_context_attribs.unwrap()(device_context, 0, context_attribs.as_ptr());

        wglMakeCurrent(device_context, wgl_context).unwrap();

        let mut game_dll = load_game_dll().unwrap();

        // after context is setup, get the render api calls
        let render_api = gengar_renderapi_opengl_windows::get_ogl_render_api();

        let mut engine_state = gengar_engine::state::State::new(resolution);
        let mut game_state = game::state::State::new();

        // setup input
        let mut input = gengar_engine::input::Input::new();

        gengar_engine::load_resources(&mut engine_state, &render_api);
        (game_dll.proc_init)(&mut game_state, &mut engine_state, &render_api);

        let mut prev_time_start: SystemTime = SystemTime::now();

        while RUNNING {
            let time_start: SystemTime = SystemTime::now();
            let prev_frame_dur: Duration = time_start.duration_since(prev_time_start).unwrap();
            prev_time_start = time_start;

            let mut message = MSG::default();

            if PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                DispatchMessageA(&message);
            }

            // check hot relaod game dll
            {
                match get_file_write_time(GAME_DLL_PATH) {
                    Ok(v) => {
                        println!("Reloding game dll");
                        FreeLibrary(game_dll.dll_handle).unwrap();
                        game_dll = load_game_dll().unwrap();
                    }
                    Err(v) => {}
                };
            }

            // Update input
            {
                input.mouse.button_left.update(MOUSE_LEFT_DOWN);
                input.mouse.button_right.update(MOUSE_RIGHT_DOWN);

                // Mouse position
                let mut cursor_info: POINT = POINT { x: 0, y: 0 };
                GetCursorPos(&mut cursor_info).unwrap();
                ScreenToClient(main_window_handle, &mut cursor_info);

                // Some windows api could be used to get this dynamically.
                let title_bar_height: f64 = 40.0;

                let prev_pos = input.mouse.pos;
                input.mouse.pos = VecTwo::new(
                    cursor_info.x as f64,
                    cursor_info.y as f64 + title_bar_height,
                );
                input.mouse.pos_delta = VecTwo::new(
                    prev_pos.x - cursor_info.x as f64,
                    prev_pos.y - (cursor_info.y as f64 + title_bar_height),
                );

                // Keyboard
                let key_states: &HashMap<KeyCode, bool> = &KEYBOARD_NEW.lock().unwrap();
                for (key, value) in key_states {
                    input
                        .keyboard
                        .entry(*key)
                        .or_insert(ButtonState::new())
                        .update(*value);
                }
            }

            // Run game / engine loops
            gengar_engine::engine_frame_start(&mut engine_state, &input, &render_api);
            (game_dll.proc_loop)(
                prev_frame_dur.as_secs_f64(),
                &mut game_state,
                &mut engine_state,
                &mut input,
            );
            gengar_engine::engine_frame_end(&mut engine_state);

            let light_trans = engine_state.transforms[game_state.light_trans.unwrap()]
                .global_matrix
                .get_position();

            render(&mut engine_state, light_trans, &resolution, &render_api);

            wglSwapLayerBuffers(device_context, gl::WGL_SWAP_MAIN_PLANE).unwrap();

            let time_end: SystemTime = SystemTime::now();
            let frame_duration: Duration = time_end.duration_since(time_start).unwrap();

            if FRAME_TARGET > frame_duration {
                let to_sleep: Duration = FRAME_TARGET - frame_duration;
                let slp = to_sleep.as_millis();
                thread::sleep(to_sleep);
            }

            {
                println!("{}mb", bytes_to_megabytes(TRACKERS[0].allocated_memory));
            }
        }
    }
}

extern "system" fn windows_callback(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_DESTROY => {
                RUNNING = false;

                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_LBUTTONDOWN => {
                MOUSE_LEFT_DOWN = true;
                LRESULT(0)
            }
            WM_LBUTTONUP => {
                MOUSE_LEFT_DOWN = false;
                LRESULT(0)
            }
            WM_RBUTTONDOWN => {
                MOUSE_RIGHT_DOWN = true;
                LRESULT(0)
            }
            WM_RBUTTONUP => {
                MOUSE_RIGHT_DOWN = false;
                LRESULT(0)
            }
            WM_KEYUP => {
                match vk_to_keycode(wparam.0) {
                    Some(keycode) => {
                        KEYBOARD_NEW.lock().unwrap().insert(keycode, false);
                    }
                    None => {}
                }

                LRESULT(0)
            }
            WM_KEYDOWN => {
                match vk_to_keycode(wparam.0) {
                    Some(keycode) => {
                        KEYBOARD_NEW.lock().unwrap().insert(keycode, true);
                    }
                    None => {}
                }

                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

extern "system" fn dummy_windows_callback(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcA(window, message, wparam, lparam) }
}

fn get_file_write_time(file_path: PCWSTR) -> std::result::Result<FILETIME, EngineError> {
    let mut file_info = WIN32_FILE_ATTRIBUTE_DATA {
        dwFileAttributes: 0,
        ftCreationTime: FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        },
        ftLastAccessTime: FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        },
        ftLastWriteTime: FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        },
        nFileSizeHigh: 0,
        nFileSizeLow: 0,
    };

    unsafe {
        let ptr = &mut file_info as *mut _ as *mut std::ffi::c_void;

        match GetFileAttributesExW(
            file_path,
            windows::Win32::Storage::FileSystem::GetFileExInfoStandard,
            ptr,
        ) {
            Ok(v) => return Ok(file_info.ftLastWriteTime),
            Err(v) => return Err(EngineError::WindowsGetFileAttributes),
        };
    }
}

unsafe fn load_game_dll() -> std::result::Result<GameDll, EngineError> {
    // Check if game dll exists, otherwise try to use the copied  _current one.
    // It could exist from previous runs
    match PathFileExistsW(GAME_DLL_PATH) {
        // original game dll does not exist
        Err(_) => match PathFileExistsW(GAME_DLL_CURRENT_PATH) {
            // Copied dll does exist, so use that.
            Ok(_) => {
                let game_dll: HMODULE = match LoadLibraryW(GAME_DLL_CURRENT_PATH) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(EngineError::WindowsLoadLibrary);
                    }
                };
                return get_game_procs_from_dll(game_dll);
            }

            // NO VALID dll exists! So problem!!
            Err(_) => {
                return Err(EngineError::MissingGameDLL);
            }
        },

        // Original dll does exist, so continue on and use that.
        Ok(_) => {}
    }

    // Create new temp dll. To allow building new original ones.
    match CopyFileW(GAME_DLL_PATH, GAME_DLL_CURRENT_PATH, false) {
        Err(v) => return Err(EngineError::WindowCopyFile),
        _ => {}
    }

    // Delete original, so that if a new original arrives then we know its new.
    match DeleteFileW(GAME_DLL_PATH) {
        Err(v) => return Err(EngineError::WindowsDeleteFile),
        _ => {}
    }

    // Load methods from library
    let game_dll: HMODULE = match LoadLibraryW(GAME_DLL_CURRENT_PATH) {
        Ok(v) => v,
        Err(e) => return Err(EngineError::WindowsLoadLibrary),
    };

    return get_game_procs_from_dll(game_dll);
}

unsafe fn get_game_procs_from_dll(dll: HMODULE) -> std::result::Result<GameDll, EngineError> {
    let init_proc = GetProcAddress(dll, s!("game_init_ogl"));
    let loop_proc = GetProcAddress(dll, s!("game_loop"));

    let dll = GameDll {
        dll_handle: dll,
        file_write_time: get_file_write_time(GAME_DLL_CURRENT_PATH)?,
        proc_init: std::mem::transmute(init_proc),
        proc_loop: std::mem::transmute(loop_proc),
    };

    Ok(dll)
}

pub fn vk_to_keycode(vk: usize) -> Option<KeyCode> {
    match vk {
        0x41 => Some(KeyCode::A),
        0x42 => Some(KeyCode::B),
        0x43 => Some(KeyCode::C),
        0x44 => Some(KeyCode::D),
        0x45 => Some(KeyCode::E),
        0x46 => Some(KeyCode::F),
        0x47 => Some(KeyCode::G),
        0x48 => Some(KeyCode::H),
        0x49 => Some(KeyCode::I),
        0x4A => Some(KeyCode::J),
        0x4B => Some(KeyCode::K),
        0x4C => Some(KeyCode::L),
        0x4D => Some(KeyCode::M),
        0x4E => Some(KeyCode::N),
        0x4F => Some(KeyCode::O),
        0x50 => Some(KeyCode::P),
        0x51 => Some(KeyCode::Q),
        0x52 => Some(KeyCode::R),
        0x53 => Some(KeyCode::S),
        0x54 => Some(KeyCode::T),
        0x55 => Some(KeyCode::U),
        0x56 => Some(KeyCode::V),
        0x57 => Some(KeyCode::W),
        0x58 => Some(KeyCode::X),
        0x59 => Some(KeyCode::Y),
        0x5A => Some(KeyCode::Z),

        0x30 => Some(KeyCode::Zero),
        0x31 => Some(KeyCode::One),
        0x32 => Some(KeyCode::Two),
        0x33 => Some(KeyCode::Three),
        0x34 => Some(KeyCode::Four),
        0x35 => Some(KeyCode::Five),
        0x36 => Some(KeyCode::Six),
        0x37 => Some(KeyCode::Seven),
        0x38 => Some(KeyCode::Eight),
        0x39 => Some(KeyCode::Nine),

        0x09 => Some(KeyCode::Tab),
        0x1B => Some(KeyCode::Escape),
        0x20 => Some(KeyCode::Spacebar),

        _ => {
            println!("Unknown keycode {:?}", vk);
            return None;
        }
    }
}
