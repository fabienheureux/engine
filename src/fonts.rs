use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    opengl::OpenGL,
    shader::Shader,
};
use gl;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use image::{DynamicImage, Luma};
use nalgebra_glm as glm;
use rusttype::{point, FontCollection, HMetrics, PositionedGlyph, Rect, Scale};
use std::collections::HashMap;
use std::os::raw::c_void;
use std::{mem, ptr};

#[derive(Debug)]
pub struct Character {
    texture_id: u32,
    bounding_box: Option<Rect<i32>>,
    height: u32,
    width: u32,
    h_metrics: HMetrics,
}

impl Character {
    /// When adding a new font, we're gonna load each glyphs in a texture buffer.
    pub fn new(glyph: &PositionedGlyph, scale: Scale) -> Self {
        let font = glyph.font().unwrap();
        let v_metrics = font.v_metrics(scale);
        let metrics = glyph.standalone().unpositioned().h_metrics();

        if glyph.pixel_bounding_box().is_none() {
            return Self {
                texture_id: 0,
                bounding_box: None,
                height: 0,
                width: 0,
                h_metrics: metrics,
            };
        }

        // Draw the glyph into the image per-pixel by using the draw closure
        let bb = glyph.pixel_bounding_box().unwrap();

        let width = bb.width() as u32;
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        let mut image = DynamicImage::new_luma8(width, height).to_luma();

        glyph.draw(|x, y, alpha| {
            image.put_pixel(
                x,
                y + bb.min.y as u32,
                Luma {
                    data: [(alpha * 255.) as u8],
                },
            )
        });

        // Load the font texture quad into opengl.
        let texture_id = OpenGL::load_glyph(
            image.width() as i32,
            image.height() as i32,
            &image.into_vec(),
        );

        Self {
            texture_id,
            bounding_box: glyph.pixel_bounding_box(),
            height,
            width,
            h_metrics: metrics,
        }
    }
}

#[derive(Debug)]
pub struct GameFont {
    vao: u32,
    vbo: u32,
    characters: HashMap<char, Character>,
}

impl GameFont {
    pub fn new() -> Self {
        let ttf = include_bytes!("../assets/fonts/OpenSans-Regular.ttf");
        let font = FontCollection::from_bytes(ttf as &[u8])
            .unwrap()
            .into_font()
            .expect("Error when turning font collection into single font.");

        let scale = Scale::uniform(32.);
        let mut characters: HashMap<char, Character> = HashMap::default();

        let alpha = "abcdefghijklmnopqrstuvwxyzéèôçñ";
        let num = "1234567890";
        let specials = "!?.,:;'(){}[]/+|_-\"\\ ";

        let chars =
            [alpha, alpha.to_uppercase().as_str(), num, specials].join("");

        let v_metrics = font.v_metrics(scale);
        let offset = point(0., v_metrics.ascent);

        let glyphs: Vec<_> =
            font.layout(chars.as_str(), scale, offset).collect();

        glyphs.iter().enumerate().for_each(|(index, g)| {
            let c = chars.chars().nth(index).unwrap();
            characters.insert(c, Character::new(g, scale));
        });

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();

        Self {
            characters,
            vao,
            vbo,
        }
    }

    pub fn gl_quad_load(&self, c: &Character, pos: (f32, f32), advance: f32) {
        let padding = 20.;

        let mut x = (pos.0 + c.h_metrics.left_side_bearing + advance);
        let mut y = pos.1;

        x += padding;
        y += padding;

        let w = c.width as f32;
        let h = c.height as f32;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices: [f32; 24] = [
            // vertex, text coord.
            x, y + h, 0.0, 0.0,
            x,  y, 0.0, 1.0,
            x + w, y, 1.0, 1.0,

            x, y + h, 0.0, 0.0,
            x + w, y, 1.0, 1.0,
            x + w, y + h, 1.0, 0.0,
        ];

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;

            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(0);
        }
    }

    pub fn get(&self, character: char) -> &Character {
        &self.characters[&character]
    }

    pub fn render(
        &self,
        text: &str,
        shader: &Shader,
        pos: (f32, f32),
        color: (f32, f32, f32),
    ) {
        OpenGL::use_shader(shader.id);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let projection =
            glm::ortho(0., SCREEN_WIDTH, 0., SCREEN_HEIGHT, -1., 1.);

        shader.set_matrix4("projection", glm::value_ptr(&projection));
        shader.set_vec3("textColor", &color);
        shader.set_int("text", 0);

        let mut advance = 0.;
        text.chars().for_each(|letter| {
            let character = self.get(letter);

            if character.bounding_box.is_some() {
                self.gl_quad_load(character, pos, advance);
            }

            advance += character.h_metrics.advance_width;

            unsafe {
                gl::BindVertexArray(self.vao);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, character.texture_id);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        });
    }
}
