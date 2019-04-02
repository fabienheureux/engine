use crate::{game_state::GameState, opengl::OpenGL};
use glutin::VirtualKeyCode;

#[derive(Default)]
pub struct Editor {
    pub enabled_physics: bool,
}

impl Editor {
    pub fn run(&mut self, state: &mut GameState) {
        let keyboard = state.window.get_keyboard_events();


        if keyboard.modifiers.shift {
            keyboard.trigger_on_press(VirtualKeyCode::P, 30, || {
                self.enabled_physics = !self.enabled_physics;
            });

            keyboard.trigger_on_press(VirtualKeyCode::L, 0, || {
                OpenGL::line_mode();
            });

            keyboard.trigger_on_press(VirtualKeyCode::F, 0, || {
                OpenGL::fill_mode();
            });

            keyboard.trigger_on_press(VirtualKeyCode::F, 0, || {
                OpenGL::fill_mode();
            });
        }
    }
}
