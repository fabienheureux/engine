use crate::opengl::OpenGL;
use std::default::Default;

pub enum Primitives {
    Plane,
    Cube,
}

#[derive(Debug)]
pub struct Mesh<'a> {
    pub vao: u32,
    pub lines: i32,
    pub has_ebo: bool,
    pub texture: Option<String>,
    pub shader: &'a str,
    pub color: (f32, f32, f32),
}

impl<'a> Mesh<'a> {
    pub fn new(
        prim: Primitives,
        texture: Option<String>,
        shader: &'a str,
    ) -> Self {
        let (vao, lines, has_ebo) = Mesh::get_gl_info(prim);

        Self {
            shader,
            vao,
            lines,
            has_ebo,
            texture,
            ..Self::default()
        }
    }

    pub fn get_gl_info(prim: Primitives) -> (u32, i32, bool) {
        match prim {
            Primitives::Plane => OpenGL::gen_plane(),
            Primitives::Cube => OpenGL::gen_cube(),
        }
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
impl<'a> Default for Mesh<'a> {
    fn default() -> Self {
        let (vao, lines, has_ebo) = OpenGL::gen_cube();
        let shader = "default";

        Self {
            vao,
            has_ebo,
            lines,
            texture: None,
            shader,
            color: (1., 1., 1.),
        }
    }
}
