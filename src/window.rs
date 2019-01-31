use crate::constants::{GAME_TITLE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::opengl::OpenGL;
use glutin::{
    dpi, ContextBuilder, DeviceEvent, ElementState, Event, EventsLoop,
    GlWindow, ModifiersState, MouseButton, VirtualKeyCode, WindowBuilder,
    WindowEvent,
};

pub struct Window {
    pub should_close: bool,
    pub event_loop: EventsLoop,
    pub gl_window: GlWindow,
    key_events: KeyEvents,
    mouse_events: MouseEvents,
}

impl Window {
    pub fn new() -> Self {
        let dimensions = dpi::LogicalSize::new(
            f64::from(SCREEN_WIDTH),
            f64::from(SCREEN_HEIGHT),
        );

        let window = WindowBuilder::new()
            .with_title(GAME_TITLE)
            .with_dimensions(dimensions);

        let context = ContextBuilder::new().with_vsync(true);
        let event_loop = EventsLoop::new();
        let gl_window = GlWindow::new(window, context, &event_loop)
            .expect("Error creating opengl window");

        gl_window
            .window()
            .grab_cursor(true)
            .expect("Error when grabbing the cursor");

        gl_window.window().hide_cursor(true);

        OpenGL::initialize(&gl_window);

        Self {
            should_close: false,
            gl_window,
            event_loop,
            key_events: KeyEvents::default(),
            mouse_events: MouseEvents::default(),
        }
    }

    pub fn capture(&mut self) {
        let mut should_close = false;
        let key_events = &mut self.key_events;
        let mut mouse_events = MouseEvents::default();

        self.event_loop
            .poll_events(|glutin_event| match &glutin_event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        should_close = true;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            if input.state == ElementState::Pressed {
                                key_events.add_keycode(keycode);
                            } else {
                                key_events.remove_keycode(keycode);
                            }

                            key_events.set_modifiers(input.modifiers);
                            should_close = keycode == VirtualKeyCode::Escape;
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        mouse_events.button = Some(*button);
                        mouse_events.state = *state;
                    }
                    _ => (),
                },
                Event::DeviceEvent { event, .. } => {
                    if let DeviceEvent::MouseMotion { delta } = event {
                        mouse_events.delta = *delta;
                        mouse_events.has_moved = true;
                    }
                }
                _ => (),
            });

        self.should_close = should_close;
        self.mouse_events = mouse_events;
    }

    pub fn trigger_on_press(
        &self,
        keycode: VirtualKeyCode,
        mut callback: impl FnMut(),
    ) {
        if self.key_events.keycodes.contains(&keycode) {
            callback();
        }
    }

    pub fn get_mouse_events(&self) -> &MouseEvents {
        &self.mouse_events
    }
}

/// Store all key events.
/// We use a vector for storing multiple key press at the same time.
pub struct KeyEvents {
    pub keycodes: Vec<VirtualKeyCode>,
    pub modifiers: ModifiersState,
}

impl KeyEvents {
    /// Update modifers (e.g: alt, ctrl).
    pub fn set_modifiers(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    /// Add keycode into the vectors only if not already there.
    pub fn add_keycode(&mut self, keycode: VirtualKeyCode) {
        if !self.keycodes.contains(&keycode) {
            self.keycodes.push(keycode);
        }
    }
    /// Remove keycode from the vectors.
    /// TODO: We should use `remove_item` method when available in stable toolchain.
    pub fn remove_keycode(&mut self, keycode: VirtualKeyCode) {
        if let Some(index) = self.keycodes.iter().position(|x| *x == keycode) {
            self.keycodes.remove(index);
        }
    }
}

impl Default for KeyEvents {
    fn default() -> Self {
        Self {
            keycodes: vec![],
            modifiers: ModifiersState::default(),
        }
    }
}

pub struct MouseEvents {
    // This is the delta of the mouse. We should use this
    // for camera stuff.
    pub delta: (f64, f64),
    // This is the cursor position in the window.
    // We should only use this to track the cursor for UI stuff.
    pub cursor_pos: dpi::LogicalPosition,
    pub state: ElementState,
    pub button: Option<MouseButton>,
    pub has_moved: bool,
}

impl Default for MouseEvents {
    fn default() -> Self {
        Self {
            delta: (0., 0.),
            cursor_pos: dpi::LogicalPosition::new(0., 0.),
            state: ElementState::Released,
            button: None,
            has_moved: false,
        }
    }
}
