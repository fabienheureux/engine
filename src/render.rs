use crate::opengl::OpenGL;
use crate::shader::Shader;
use crate::time::Time;
use crate::window::Window;
use gl;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use nalgebra_glm as glm;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use crate::camera::Camera;
use glutin::VirtualKeyCode;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Render {
    shader: Shader,
    vao: u32,
}

impl Render {
    pub fn new() -> Self {
        let vertices: [f32; 20] = [
            0.5, 0.5, 0., 1., 1., 0.5, -0.5, 0., 1., 0., -0.5, -0.5, 0., 0.,
            0., -0.5, 0.5, 0., 0., 1.,
        ];
        let indices: [i32; 6] = [0, 1, 3, 1, 2, 3];

        let current_shader = Shader::new("shaders", "default_cube");
        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();
        let ebo = OpenGL::gen_buffer();

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

        Self {
            shader: current_shader,
            // gl_window,
            vao,
        }
    }

    pub fn draw(
        &self,
        time: &Time,
        tex_id: u32,
        window: &Window,
        cam: &mut Camera,
    ) {
        let identity = glm::Mat4::identity();

        let model = glm::rotate_x(&identity, -45_f32.to_radians());
        let projection = glm::perspective(
            45_f32.to_radians(),
            (SCREEN_WIDTH / SCREEN_HEIGHT) as f32,
            0.1,
            100.,
        );

        let speed = 2.5;
        let speed = speed * time.dt as f32;

        window.trigger_on_press(VirtualKeyCode::W, || {
            cam.position += speed * cam.front;
        });

        window.trigger_on_press(VirtualKeyCode::S, || {
            cam.position -= speed * cam.front;
        });

        window.trigger_on_press(VirtualKeyCode::D, || {
            cam.position += glm::normalize(&cam.front.cross(&cam.up)) * speed;
        });

        window.trigger_on_press(VirtualKeyCode::A, || {
            cam.position -= glm::normalize(&cam.front.cross(&cam.up)) * speed;
        });

        window.trigger_on_press(VirtualKeyCode::Q, || {
            cam.position -= speed * cam.up;
        });

        window.trigger_on_press(VirtualKeyCode::E, || {
            cam.position += speed * cam.up;
        });

        let view =
            glm::look_at(&cam.position, &(cam.position + cam.front), &cam.up);

        self.shader.set_matrix4("model", glm::value_ptr(&model));
        self.shader.set_matrix4("view", glm::value_ptr(&view));
        self.shader
            .set_matrix4("projection", glm::value_ptr(&projection));

        unsafe {
            // Clear color buffer with the color specified by gl::ClearColor.
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(self.shader.id);
            self.shader.set_int("ourTexture", tex_id as i32);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);

            // let green_value = time.now_to_secs().sin() / 2. + 0.5;
            self.shader.set_uniform4f("ourColor", &(0., 1., 0., 1.));

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
