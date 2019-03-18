use crate::{opengl::OpenGL, shader::Shader};
use std::default::Default;

pub enum Primitives {
    Plane,
    Cube,
}

#[derive(Debug)]
pub struct Mesh {
    pub vao: u32,
    pub has_ebo: bool,
    pub texture: Option<String>,
    pub shader: Shader,
    pub color: (f32, f32, f32),
}

impl Mesh {
    pub fn new(
        prim: Primitives,
        texture: Option<String>,
        (vert, frag): (&str, &str),
    ) -> Self {
        let shader = Mesh::set_shader(vert, frag);
        let (vao, has_ebo) = Mesh::get_gl_info(prim);

        Self {
            shader,
            vao,
            has_ebo,
            texture,
            ..Self::default()
        }
    }

    pub fn get_gl_info(prim: Primitives) -> (u32, bool) {
        match prim {
            Primitives::Plane => OpenGL::gen_plane(),
            Primitives::Cube => OpenGL::gen_cube(),
        }
    }

    pub fn set_shader(vert: &str, frag: &str) -> Shader {
        let shader = Shader::new().with_vert(vert).with_frag(frag);

        OpenGL::set_uniform_block(shader.id, 0, "Camera");
        OpenGL::set_uniform_block(shader.id, 1, "Lights");

        shader
    }

    pub fn get_shader(&self) -> &Shader {
        &self.shader
    }

    pub fn get_vao(&self) -> u32 {
        self.vao
    }

    pub fn get_texture(&self) -> Option<&String> {
        self.texture.as_ref()
    }
}

// The default function will create a simple cube mesh without
// with only a color.
impl Default for Mesh {
    fn default() -> Self {
        let (vao, has_ebo) = OpenGL::gen_cube();
        let shader = Shader::new().with_vert("default").with_frag("default");

        Self {
            vao,
            has_ebo,
            texture: None,
            shader,
            color: (1., 1., 1.),
        }
    }
}
