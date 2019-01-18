use gl;
use glutin::{GlContext, GlWindow};

pub struct OpenGL;

impl OpenGL {
    pub fn initialize(gl_window: &GlWindow) {
        unsafe {
            gl_window
                .make_current()
                .expect("Error setting the current context");

            gl::load_with(|symbol| {
                gl_window.get_proc_address(symbol) as *const _
            });

            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        }
    }
}
