use crate::opengl::OpenGL;
use glutin::{ContextBuilder, EventsLoop, GlWindow, WindowBuilder};

pub struct Window {
    pub event_loop: EventsLoop,
    pub gl_window: GlWindow,
}

impl Window {
    pub fn new(window_title: &str) -> Self {
        let window = WindowBuilder::new().with_title(window_title);
        let context = ContextBuilder::new();
        let event_loop = EventsLoop::new();
        let gl_window = GlWindow::new(window, context, &event_loop)
            .expect("Error creating opengl window");

        OpenGL::initialize(&gl_window);

        Self {
            gl_window,
            event_loop,
        }
    }
}
