use crate::{opengl::OpenGL, shader::Shader, texture::Texture};
use std::default::Default;

type RGB = (f32, f32, f32);

#[derive(Debug)]
pub struct Material {
    pub texture: Option<Texture>,
    pub shader: Option<Shader>,
    diffuse: i32,
    specular: RGB,
    shininess: f32,
}

impl Material {
    pub fn add_texture(&mut self, texture_path: &str) {
        let mut texture = Texture::new(texture_path);
        texture.generate_texture();

        self.texture = Some(texture);
    }

    pub fn add_shader(&mut self, vert: &str, frag: &str) {
        let shader = Shader::new().with_vert(vert).with_frag(frag);
        self.shader = Some(shader);
    }

    pub fn get_shader(&self) -> &Shader {
        self.shader.as_ref().expect("No shader found.")
    }

    pub fn get_texture(&self) -> &Texture {
        self.texture.as_ref().expect("No texture found.")
    }

    pub fn send_to_shader(&self, shader: &Shader) {
        if let Some(texture) = &self.texture {
            shader.set_int("material.diffuse", texture.id as i32);
        }

        shader.set_vec3("material.specular", &self.specular);
        shader.set_float("material.shininess", self.shininess);
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            texture: None,
            shader: None,
            diffuse: 0,
            specular: (0.5, 0.5, 0.5),
            shininess: 32.,
        }
    }
}
