use crate::{opengl::OpenGL, shader::Shader, world::Renderer};
use nalgebra_glm as glm;
use std::ptr;

type Pos = glm::TVec3<f32>;

#[derive(Debug)]
pub struct Plane {
    vao: u32,
    texture_id: u32,
    shader: Shader,
    position: Pos,
}

impl Plane {
    pub fn new(position: Pos) -> Self {
        let shader = Shader::new()
            .with_vert("default_cube")
            .with_frag("default_cube");

        let (vao, texture_id, position) = OpenGL::gen_plane(position);

        Self {
            vao,
            texture_id,
            shader,
            position,
        }
    }
}

impl Renderer for Plane {
    fn draw(&self, model: glm::Mat4, view: glm::Mat4, proj: glm::Mat4) {
        unsafe { gl::UseProgram(self.shader.id) }

        let model = glm::translate(&model, &self.position);
        self.shader.set_matrix4("model", glm::value_ptr(&model));
        self.shader.set_matrix4("view", glm::value_ptr(&view));
        self.shader.set_matrix4("projection", glm::value_ptr(&proj));

        self.shader.set_int("ourTexture", self.texture_id as i32);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

#[derive(Debug)]
pub struct Cube {
    vao: u32,
    texture_id: u32,
    shader: Shader,
    position: Pos,
}

impl Cube {
    pub fn new(position: Pos) -> Self {
        let shader = Shader::new()
            .with_vert("default_cube")
            .with_frag("default_cube");

        let (vao, texture_id, position) = OpenGL::gen_cube(position);

        Self {
            vao,
            texture_id,
            shader,
            position,
        }
    }
}

impl Renderer for Cube {
    fn draw(&self, model: glm::Mat4, view: glm::Mat4, proj: glm::Mat4) {
        unsafe { gl::UseProgram(self.shader.id) }

        let model = glm::translate(&model, &self.position);
        self.shader.set_matrix4("model", glm::value_ptr(&model));
        self.shader.set_matrix4("view", glm::value_ptr(&view));
        self.shader.set_matrix4("projection", glm::value_ptr(&proj));

        self.shader.set_int("ourTexture", self.texture_id as i32);

        self.shader.set_color("lightPos", &(1.2, 1., 2.));
        self.shader.set_color("objectColor", &(1., 0.5, 0.31));
        self.shader.set_color("lightColor", &(1., 1., 1.));

        unsafe {
            gl::UseProgram(self.shader.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::BindVertexArray(self.vao);

            // No EBO for this cube.
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}

#[derive(Debug)]
pub struct LightSource {
    vao: u32,
    texture_id: u32,
    shader: Shader,
    position: Pos,
}

impl LightSource {
    pub fn new(position: Pos) -> Self {
        let shader = Shader::new()
            .with_vert("default_cube")
            .with_frag("light_source");

        let (vao, texture_id, position) = OpenGL::gen_cube(position);

        Self {
            vao,
            texture_id,
            shader,
            position,
        }
    }
}

impl Renderer for LightSource {
    fn draw(&self, model: glm::Mat4, view: glm::Mat4, proj: glm::Mat4) {
        unsafe { gl::UseProgram(self.shader.id) }

        let mut model = glm::translate(&model, &self.position);
        model = glm::scale(&model, &glm::vec3(0.3, 0.3, 0.3));

        self.shader.set_matrix4("view", glm::value_ptr(&view));
        self.shader.set_matrix4("projection", glm::value_ptr(&proj));
        self.shader.set_matrix4("model", glm::value_ptr(&model));

        unsafe {
            gl::UseProgram(self.shader.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::BindVertexArray(self.vao);

            // No EBO for this cube.
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
