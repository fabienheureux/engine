use crate::{opengl::OpenGL, shader::Shader, texture::Texture};

pub enum Primitives {
    Plane,
    Cube,
}

#[derive(Default, Debug)]
pub struct Mesh {
    pub vao: u32,
    pub has_ebo: bool,
    pub texture: Option<Texture>,
    pub shader: Option<Shader>,
}

impl Mesh {
    pub fn add_primitive(&mut self, prim: Primitives) {
        let (vao, has_ebo) = match prim {
            Primitives::Plane => OpenGL::gen_plane(),
            Primitives::Cube => OpenGL::gen_cube(),
        };

        self.vao = vao;
        self.has_ebo = has_ebo;
    }

    pub fn add_texture(&mut self, texture_path: &str) {
        let mut texture = Texture::new(texture_path);
        texture.generate_texture();
        self.texture = Some(texture);
    }

    pub fn add_shader(&mut self) {
        let shader = Shader::new()
            .with_vert("default_cube")
            .with_frag("default_cube");

        OpenGL::set_uniform_block(shader.id, 0, "Camera");
        OpenGL::set_uniform_block(shader.id, 1, "Lights");

        self.shader = Some(shader);
    }

    pub fn get_shader(&self) -> &Shader {
        self.shader.as_ref().unwrap()
    }

    pub fn get_vao(&self) -> u32 {
        self.vao
    }

    pub fn get_texture(&self) -> &Texture {
        self.texture.as_ref().unwrap()
    }
}
