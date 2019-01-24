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
        let mut key_events = KeyEvents::default();
        let mut mouse_events = MouseEvents::default();

        self.event_loop
            .poll_events(|glutin_event| match &glutin_event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        should_close = true;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.virtual_keycode.unwrap()
                            == VirtualKeyCode::Escape
                        {
                            should_close = true;
                        }

                        key_events.keycode = input.virtual_keycode;
                        key_events.state = input.state;
                        key_events.modifiers = input.modifiers;
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        mouse_events.button = Some(*button);
                        mouse_events.state = *state;
                    }
                    WindowEvent::CursorMoved {
                        position,
                        modifiers,
                        ..
                    } => {
                        mouse_events.cursor_pos = *position;
                        mouse_events.modifiers = *modifiers;
                    }
                    WindowEvent::CursorEntered { .. } => {
                        mouse_events.cursor_in_window = true;
                    }
                    WindowEvent::CursorLeft { .. } => {
                        mouse_events.cursor_in_window = false;
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
        self.key_events = key_events;
        self.mouse_events = mouse_events;
    }

    pub fn trigger_on_press(
        &self,
        keycode: VirtualKeyCode,
        mut callback: impl FnMut(),
    ) {
        let is_pressed = self.key_events.state == ElementState::Pressed;
        let is_keycode = self.key_events.keycode == Some(keycode);

        if is_keycode && is_pressed {
            callback();
        }
    }

    pub fn get_mouse_events(&self) -> &MouseEvents {
        &self.mouse_events
    }
}

#[derive(Clone)]
pub struct KeyEvents {
    pub keycode: Option<VirtualKeyCode>,
    pub modifiers: ModifiersState,
    pub state: ElementState,
}

impl Default for KeyEvents {
    fn default() -> Self {
        Self {
            state: ElementState::Released,
            keycode: None,
            modifiers: ModifiersState::default(),
        }
    }
}

#[derive(Clone)]
pub struct MouseEvents {
    // This is the delta of the mouse. We should use this
    // for camera stuff.
    pub delta: (f64, f64),
    // This is the cursor position in the window.
    // We should only use this to track the cursor for UI stuff.
    pub cursor_pos: dpi::LogicalPosition,
    pub modifiers: ModifiersState,
    pub state: ElementState,
    pub button: Option<MouseButton>,
    pub cursor_in_window: bool,
    pub has_moved: bool,
}

impl Default for MouseEvents {
    fn default() -> Self {
        Self {
            delta: (0., 0.),
            cursor_pos: dpi::LogicalPosition::new(0., 0.),
            modifiers: ModifiersState::default(),
            state: ElementState::Released,
            button: None,
            cursor_in_window: false,
            has_moved: false,
        }
    }
}
