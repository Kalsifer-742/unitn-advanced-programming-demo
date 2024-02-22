use macroquad::input::KeyCode;

pub(super) struct KeyboardControls {
    pub(super) exit: KeyCode,
    pub(super) toggle_free_mouse: KeyCode,
    pub(super) move_forward: KeyCode,
    pub(super) move_backward: KeyCode,
    pub(super) move_left: KeyCode,
    pub(super) move_right: KeyCode,
    pub(super) move_up: KeyCode,
    pub(super) move_down: KeyCode,
    pub(super) toggle_tile_info: KeyCode,
    pub(super) toggle_help: KeyCode,
    pub(super) toggle_statistics: KeyCode,
    pub(super) toggle_hud: KeyCode,
    //pub(super) take_screenshot: KeyCode
}

impl Default for KeyboardControls {
    fn default() -> Self {
        Self {
            exit: KeyCode::Escape,
            toggle_free_mouse: KeyCode::G,
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_up: KeyCode::Space,
            move_down: KeyCode::LeftShift,
            toggle_tile_info: KeyCode::I,
            toggle_help: KeyCode::H,
            toggle_statistics: KeyCode::F3,
            toggle_hud: KeyCode::C,
            //take_screenshot: KeyCode::F2
        }
    }
}
