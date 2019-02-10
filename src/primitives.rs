use crate::{
    camera::Camera, opengl::OpenGL, renderer::Renderer, shader::Shader,
    time::Time,
};
use nalgebra_glm as glm;
use std::ptr;

type Pos = glm::TVec3<f32>;

#[derive(Debug)]
pub struct Plane {
    vao: u32,
    texture_id: u32,
    pub shader: Shader,
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
    fn draw(
        &mut self,
        _time: &Time,
        _cam: &Camera,
        view: glm::Mat4,
        proj: glm::Mat4,
    ) {
        unsafe { gl::UseProgram(self.shader.id) }

        let model = glm::Mat4::identity();
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
    fn draw(
        &mut self,
        time: &Time,
        camera: &Camera,
        view: glm::Mat4,
        proj: glm::Mat4,
    ) {
        unsafe { gl::UseProgram(self.shader.id) }

        let model = glm::Mat4::identity();
        let model = glm::translate(&model, &self.position);

        self.shader.set_matrix4("model", glm::value_ptr(&model));
        self.shader.set_matrix4("view", glm::value_ptr(&view));
        self.shader.set_matrix4("projection", glm::value_ptr(&proj));

        let center: glm::TVec3<f32> = self.position;
        let r: f32 = 1.5;
        let mut position = glm::vec3(0., 0., 0.);
        let now = time.now_to_secs();
        position.x = center.x + (r * now.cos() as f32);
        position.z = center.z + (r * now.sin() as f32);

        let cam_pos = camera.position;

        self.shader
            .set_vec3("viewPos", &(cam_pos.x, cam_pos.y, cam_pos.z));
        self.shader
            .set_vec3("light.position", &(position.x, position.y, position.z));

        self.shader.set_vec3("light.ambient", &(0.2, 0.2, 0.2));
        self.shader.set_vec3("light.diffuse", &(0.5, 0.5, 0.5));
        self.shader.set_vec3("light.specular", &(1., 1., 1.));

        self.shader
            .set_int("material.diffuse", self.texture_id as i32);
        self.shader.set_vec3("material.specular", &(0.5, 0.5, 0.5));
        self.shader.set_float("material.shininess", 32.);

        unsafe {
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
    fn draw(
        &mut self,
        time: &Time,
        _cam: &Camera,
        view: glm::Mat4,
        proj: glm::Mat4,
    ) {
        unsafe { gl::UseProgram(self.shader.id) }

        let model = glm::Mat4::identity();
        let center: glm::TVec3<f32> = glm::vec3(0., 0., 0.);
        let rayon: f32 = 2.5;
        let now = time.now_to_secs();
        self.position.x = center.x + (rayon * now.cos() as f32);
        self.position.z = center.z + (rayon * now.sin() as f32);

        let mut model = glm::translate(&model, &self.position);
        model = glm::scale(&model, &glm::vec3(0.3, 0.3, 0.3));

        self.shader.set_matrix4("view", glm::value_ptr(&view));
        self.shader.set_matrix4("projection", glm::value_ptr(&proj));
        self.shader.set_matrix4("model", glm::value_ptr(&model));

        self.shader.set_vec3(
            "light.position",
            &(self.position.x, self.position.y, self.position.z),
        );
        self.shader.set_vec3("light.ambient", &(0.2, 0.2, 0.2));
        self.shader.set_vec3("light.diffuse", &(0.5, 0.5, 0.5));
        self.shader.set_vec3("light.specular", &(1., 1., 1.));

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::BindVertexArray(self.vao);

            // No EBO for this cube.
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
