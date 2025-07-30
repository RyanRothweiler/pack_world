use std::ffi::c_void;

fn main() {
    let mut game_state = example_game::GameState { state_here: 100 };

    elara_platform_windows::platform_main("example_game", &mut game_state as *mut _ as *mut c_void);

    println!("Hello, world!");
}
