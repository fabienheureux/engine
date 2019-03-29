use crate::{opengl::OpenGL, shader::Shader};
use gl;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use image::{DynamicImage, Rgba};
use nalgebra_glm as glm;
use rusttype::{
    point, Font, FontCollection, HMetrics, PositionedGlyph, Rect, Scale,
};
use std::collections::HashMap;
use std::os::raw::c_void;
use std::{mem, ptr};

#[derive(Debug)]
pub struct Character {
    texture_id: u32,
    size: glm::TVec2<f32>,
    bounding_box: Rect<f32>,
    h_metrics: HMetrics,
}

impl Character {
    pub fn new(character: &str, height: f32, font: &Font) -> Self {
        // 2x scale in x direction to counter the aspect ratio of monospace characters.
        let scale = Scale {
            x: height * 2.0,
            y: height,
        };

        let v_metrics = font.v_metrics(scale);
        let offset = point(0., 0.);

        let glyphs: Vec<PositionedGlyph> =
            font.layout(character, scale, offset).collect();

        // work out the layout size
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        // Loop through the glyphs in the text, positing each one on a line
        let glyph = glyphs.first().unwrap();

        // Draw the glyph into the image per-pixel by using the draw closure
        let width = glyph.pixel_bounding_box().unwrap().max.x as u32;

        let mut image = DynamicImage::new_rgba8(width, height).to_rgba();

        glyph.draw(|x, y, v| {
            if character == "H" {
                dbg!((x, y, v));
            }

            image.put_pixel(
                x,
                y,
                Rgba {
                    data: [255, 0, 0, (v * 255.) as u8],
                },
            )
        });

        let metrics = glyph.standalone().unpositioned().h_metrics();

        if character == "H" {
            dbg!(metrics);
            dbg!(glyph.pixel_bounding_box().unwrap());
            dbg!(image.width());
            dbg!(image.height());
        }

        // Load the font texture quad into opengl.
        let texture_id = OpenGL::load_glyph(
            image.width() as i32,
            image.height() as i32,
            &image.into_vec(),
        );

        //
        // Useful for debugging...
        //
        // let path = format!("assets/test_{}.png", character);
        // image.save(path).unwrap();

        Self {
            texture_id,
            bounding_box: glyph.unpositioned().exact_bounding_box().unwrap(),
            size: glm::vec2(width as f32, height as f32),
            h_metrics: metrics,
        }
    }
}

#[derive(Debug)]
pub struct GameFont {
    vao: u32,
    vbo: u32,
    characters: HashMap<String, Character>,
}

impl GameFont {
    pub fn new() -> Self {
        let ttf = include_bytes!("../assets/fonts/Merriweather-Regular.ttf");
        let font = FontCollection::from_bytes(ttf as &[u8])
            .unwrap()
            .into_font()
            .expect("Error when turning font collection into single font.");

        let height = 12.4;
        let mut characters: HashMap<String, Character> = HashMap::default();

        // TODO: Iterate over all the glyphs inside the font.
        // I don't want to iterate only the alphabet here, but rather iterate
        // over all the glyphs contained in the font.
        let alpha = "a b c d e f g h i j k l m n o p q r s t u w x y z";
        alpha.split_whitespace().for_each(|c| {
            characters
                .insert(String::from(c), Character::new(c, height, &font));
        });

        let vao = OpenGL::gen_vao();
        let vbo = OpenGL::gen_buffer();

        Self {
            characters,
            vao,
            vbo,
        }
    }

    pub fn gl_quad_load(&self, c: &Character) {
        let scale = 1.;

        let x = 0. * scale;
        let y = 0. * scale;

        let w = c.size.x * scale;
        let h = c.size.y * scale;

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
                gl::STATIC_DRAW,
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

    pub fn get(&self, character: &str) -> &Character {
        &self.characters[character]
    }

    pub fn render(&self, _text: &str, shader: &Shader, color: (f32, f32, f32)) {
        OpenGL::use_shader(shader.id);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        shader.set_vec3("textColor", &color);
        shader.set_int("text", 0);

        let letter = "a";
        let c = self.get(letter);

        self.gl_quad_load(c);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, c.texture_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}
