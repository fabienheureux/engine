use crate::{game_state::GameState, opengl::OpenGL};
use glutin::VirtualKeyCode;

#[derive(Default)]
pub struct Editor;

impl Editor {
    pub fn run(state: &GameState) {
        let keyboard = state.window.get_keyboard_events();

        if keyboard.modifiers.shift {
            keyboard.trigger_on_press(VirtualKeyCode::L, || {
                OpenGL::line_mode();
            });

            keyboard.trigger_on_press(VirtualKeyCode::F, || {
                OpenGL::fill_mode();
            });
        }
    }
}
