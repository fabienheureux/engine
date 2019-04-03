use crate::constants::{GAME_TITLE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::opengl::OpenGL;
use glutin::{
    dpi, ContextBuilder, DeviceEvent, ElementState, Event, EventsLoop,
    GlWindow, ModifiersState, MouseButton, VirtualKeyCode, WindowBuilder,
    WindowEvent,
};
use std::collections::HashMap;
use std::time::Instant;

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

        OpenGL::initialize(&gl_window);

        Self {
            should_close: false,
            gl_window,
            event_loop,
            key_events: KeyEvents::default(),
            mouse_events: MouseEvents::default(),
        }
    }

    pub fn swap_gl(&self) {
        self.gl_window
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }

    /// Hide and Grab the cursor.
    pub fn hide_cursor(&self, is_hide: bool) {
        self.gl_window
            .window()
            .grab_cursor(is_hide)
            .expect("Error when grabbing the cursor");

        self.gl_window.window().hide_cursor(is_hide);
    }

    pub fn capture(&mut self) {
        let mut should_close = false;
        let key_events = &mut self.key_events;
        let mouse_events = &mut self.mouse_events;
        mouse_events.has_moved = false;

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
                        mouse_events.is_pressed =
                            *state == ElementState::Pressed;
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
    }

    pub fn get_mouse_events(&self) -> &MouseEvents {
        &self.mouse_events
    }

    pub fn get_keyboard_events(&mut self) -> &mut KeyEvents {
        &mut self.key_events
    }
}

/// Used to store information when a specified key is pressed.
/// The `once` field is used for all press once methods.
pub struct KeyState {
    time: Instant,
    once: bool,
}

/// Store all key events.
/// We use a hashmap for storing multiple key press at the same time.
pub struct KeyEvents {
    pub keycodes: HashMap<VirtualKeyCode, KeyState>,
    pub modifiers: ModifiersState,
}

impl KeyEvents {
    /// Will call only once the closure when the given closure is pressed.
    pub fn once(
        &mut self,
        keycode: VirtualKeyCode,
        mut callback: impl FnMut(),
    ) {
        if let Some(pressed) = self.keycodes.get_mut(&keycode) {
            if !pressed.once {
                pressed.once = true;
                callback();
            }
        }
    }

    /// Will call the closure only if the given keycode is pressed.
    pub fn pressed(
        &mut self,
        keycode: VirtualKeyCode,
        mut callback: impl FnMut(),
    ) {
        if let Some(value) = self.keycodes.get_mut(&keycode) {
            value.time = Instant::now();
            callback();
        }
    }

    /// Update modifers (e.g: alt, ctrl).
    pub fn set_modifiers(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    /// Add keycode if not already there.
    pub fn add_keycode(&mut self, keycode: VirtualKeyCode) {
        self.keycodes.entry(keycode).or_insert_with(|| KeyState {
            time: Instant::now(),
            once: false,
        });
    }

    /// Remove specified keycode.
    /// TODO: We should use `remove_item` method when available
    /// in the stable toolchain.
    pub fn remove_keycode(&mut self, keycode: VirtualKeyCode) {
        if self.keycodes.contains_key(&keycode) {
            self.keycodes.remove(&keycode);
        }
    }
}

impl Default for KeyEvents {
    fn default() -> Self {
        Self {
            keycodes: HashMap::new(),
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
    pub is_pressed: bool,
    pub button: Option<MouseButton>,
    pub has_moved: bool,
}

impl MouseEvents {
    /// Will call the closure only if the given mouse button is pressed.
    pub fn pressed(&self, button: MouseButton, mut callback: impl FnMut()) {
        if let Some(pressed_button) = self.button {
            if pressed_button == button && self.is_pressed {
                callback();
            }
        }
    }

    /// Will call the closure only if the given mouse button is released.
    pub fn trigger_on_release(
        &self,
        button: MouseButton,
        mut callback: impl FnMut(),
    ) {
        if let Some(pressed_button) = self.button {
            if pressed_button == button && !self.is_pressed {
                callback();
            }
        }
    }
}

impl Default for MouseEvents {
    fn default() -> Self {
        Self {
            delta: (0., 0.),
            cursor_pos: dpi::LogicalPosition::new(0., 0.),
            is_pressed: false,
            button: None,
            has_moved: false,
        }
    }
}
