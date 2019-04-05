use crate::game_state::GameState;
use glutin::VirtualKeyCode;
use std::default::Default;

pub struct Editor {
    pub enabled_physics: bool,
    pub enabled_wireframe_mode: bool,
}

impl Editor {
    pub fn check_inputs(&mut self, state: &mut GameState) {
        let keyboard = state.window.get_keyboard_events();

        if keyboard.modifiers.shift {
            keyboard.once(VirtualKeyCode::P, || {
                self.enabled_physics = !self.enabled_physics;
            });

            keyboard.once(VirtualKeyCode::L, || {
                self.enabled_wireframe_mode = !self.enabled_wireframe_mode;
            });
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            enabled_physics: true,
            enabled_wireframe_mode: false,
        }
    }
}
