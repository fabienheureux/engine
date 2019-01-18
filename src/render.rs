use crate::shader::Shader;
use crate::time::Time;
use gl;
use gl::types::{GLfloat, GLsizeiptr, GLsizei};
use glutin::GlWindow;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct Render<'a> {
    shader: Shader,
    gl_window: &'a GlWindow,
    vao: u32,
}

impl<'a> Render<'a> {
    pub fn new(gl_window: &'a GlWindow) -> Self {
        let vertices: [f32; 20] = [
            0.5, 0.5, 0., 1., 1., 0.5, -0.5, 0., 1., 0., -0.5, -0.5, 0., 0.,
            0., -0.5, 0.5, 0., 0., 1.,
        ];

        let indices: [i32; 6] = [0, 1, 3, 1, 2, 3];

        let current_shader = Shader::new("assets/shaders", "default_cube");
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<GLfloat>() as GLsizei,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }

        Self {
            shader: current_shader,
            gl_window,
            vao,
        }
    }

    pub fn draw(&self, _time: &Time, tex_id: u32) {
        unsafe {
            // Clear color buffer with the color specified by gl::ClearColor.
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.shader.id);

            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            //
            // let green_value = time.now_to_secs().sin() / 2. + 0.5;
            // self.shader
            //     .set_uniform4f("ourColor", &(0., green_value as f32, 0., 1.));

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        self.gl_window
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }
}
