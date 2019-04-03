use crate::{game_state::GameState, opengl::OpenGL};
use glutin::VirtualKeyCode;

#[derive(Default)]
pub struct Editor {
    pub enabled_physics: bool,
}

impl Editor {
    pub fn new(enabled_physics: bool) -> Self {
        Self { enabled_physics }
    }

    pub fn run(&mut self, state: &mut GameState) {
        let keyboard = state.window.get_keyboard_events();

        if keyboard.modifiers.shift {
            keyboard.once(VirtualKeyCode::P, || {
                self.enabled_physics = !self.enabled_physics;
            });

            keyboard.once(VirtualKeyCode::L, || {
                OpenGL::line_mode();
            });

            keyboard.once(VirtualKeyCode::F, || {
                OpenGL::fill_mode();
            });

            keyboard.once(VirtualKeyCode::F, || {
                OpenGL::fill_mode();
            });
        }
    }
}
