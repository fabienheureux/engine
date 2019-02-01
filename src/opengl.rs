use crate::texture::Texture;
use gl;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glutin::{GlContext, GlWindow};
use nalgebra_glm as glm;
use std::os::raw::c_void;
use std::{mem, ptr};

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

    // Create a little plane.
    // Return vao and texture id and positions.
    pub fn gen_plane(pos: glm::TVec3<f32>) -> (u32, u32, glm::TVec3<f32>) {
        let vertices: [f32; 20] = [
            0.5, 0.5, 0., 1., 1., 0.5, -0.5, 0., 1., 0., -0.5, -0.5, 0., 0.,
            0., -0.5, 0.5, 0., 0., 1.,
        ];
        let indices: [i32; 6] = [0, 1, 3, 1, 2, 3];

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();
        let ebo = OpenGL::gen_buffer();

        let mut t = Texture::new("./assets/textures/wall.jpg");
        t.generate_texture();

        unsafe {
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

        (vao, t.texture_id, pos)
    }

    // Create a little cube
    // Return vao and texture id and positions.
    pub fn gen_cube(pos: glm::TVec3<f32>) -> (u32, u32, glm::TVec3<f32>) {
        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5,
            -0.5, 1.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0,
            1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, 0.5,
            -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0,
            1.0, -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5,
            0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5,
            0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5,
            1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5,
            -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0,
            0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5,
            -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5,
            1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5,
            0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
        ];
        // let indices: [i32; 6] = [0, 1, 3, 1, 2, 3];

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();
        // let ebo = OpenGL::gen_buffer();

        let mut t = Texture::new("./assets/textures/wall.jpg");
        t.generate_texture();

        unsafe {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // gl::BufferData(
            //     gl::ELEMENT_ARRAY_BUFFER,
            //     (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            //     &indices[0] as *const i32 as *const c_void,
            //     gl::STATIC_DRAW,
            // );

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

        (vao, t.texture_id, pos)
    }
}
