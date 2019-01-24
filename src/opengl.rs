use gl;
use glutin::{GlContext, GlWindow};

// This is just a namespace for now.
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

    /// Generate Vertex Array Object.
    pub fn gen_vao() -> u32 {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) }
        vao
    }

    /// Generate buffer.
    pub fn gen_buffer() -> u32 {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        id
    }
}
