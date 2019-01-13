use crate::shader::Shader;
use crate::time::Time;
use gl;
use gl::types::*;
use glutin::{GlContext, GlWindow};
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct Render {
    shader: Shader,
    gl_window: GlWindow,
    vao: u32,
}

impl Render {
    pub fn new(gl_window: GlWindow) -> Self {
        unsafe {
            gl::load_with(|symbol| {
                gl_window.get_proc_address(symbol) as *const _
            });

            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];

        let current_shader = Shader::new("assets/shaders", "default_cube");
        let (mut vao, mut vbo) = (0, 0);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            shader: current_shader,
            gl_window,
            vao,
        }
    }

    pub fn draw(&self, time: &Time) {
        unsafe {
            // Clear color buffer with the color specified by gl::ClearColor.
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.shader.id);

            let green_value = time.now_to_secs().sin() / 2. + 0.5;

            self.shader
                .set_uniform4f("ourColor", &(0., green_value as f32, 0., 1.));

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        self.gl_window
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }
}
