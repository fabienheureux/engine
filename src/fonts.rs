use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    opengl::OpenGL,
    shader::Shader,
};
use gl;
use image::{DynamicImage, Luma};
use nalgebra_glm as glm;
use rusttype::{point, FontCollection, HMetrics, PositionedGlyph, Rect, Scale};
use std::collections::HashMap;

#[derive(Debug)]
struct GpuInfo {
    texture_id: u32,
    height: u32,
    width: u32,
    bounding_box: Rect<i32>,
}

#[derive(Debug)]
pub struct Character {
    gpu_info: Option<GpuInfo>,
    h_metrics: HMetrics,
}

impl Character {
    /// When adding a new font, we're gonna load each glyphs in a texture buffer.
    /// We are considering the glyph as a whitespace if it hasn't a bounding box.
    pub fn new(glyph: &PositionedGlyph, scale: Scale) -> Self {
        let font = glyph.font().unwrap();
        let v_metrics = font.v_metrics(scale);
        let h_metrics = glyph.unpositioned().h_metrics();
        let bounding_box = glyph.pixel_bounding_box();
        let mut gpu_info = None;

        if let Some(bb) = bounding_box {
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

            gpu_info = Some(GpuInfo {
                texture_id,
                height,
                width,
                bounding_box: bb,
            });
        }

        Self {
            gpu_info,
            h_metrics,
        }
    }
}

#[derive(Debug)]
/// We are using only one quad for all the glyph. So when the glyph is
/// render, we update the vertex buffer.
pub struct GameFont {
    vao: u32,
    vbo: u32,
    characters: HashMap<char, Character>,
}

impl GameFont {
    pub fn new(scale: f32) -> Self {
        let ttf = include_bytes!("../assets/fonts/Merriweather-Regular.ttf");
        let font = FontCollection::from_bytes(ttf as &[u8])
            .unwrap()
            .into_font()
            .expect("Error when turning font collection into single font.");

        let scale = Scale::uniform(scale);
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
            let c = chars
                .chars()
                .nth(index)
                .expect("Char not found in the glyph.");

            characters.insert(c, Character::new(g, scale));
        });

        let (vao, vbo) = OpenGL::create_font_quad();

        Self {
            characters,
            vao,
            vbo,
        }
    }

    pub fn get(&self, character: char) -> &Character {
        &self.characters[&character]
    }

    /// We are using all pixel coord so we use a ortho projection with screen
    /// size.
    pub fn render(
        &self,
        text: &str,
        shader: &Shader,
        (x, y): (f32, f32),
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
            let padding = 20.;

            // We don't want to render anything if it's a whitespace.
            if let Some(gpu) = character.gpu_info.as_ref() {
                let y = y + padding;
                let x = x
                    + character.h_metrics.left_side_bearing
                    + advance
                    + padding;

                let w = gpu.width as f32;
                let h = gpu.height as f32;

                OpenGL::update_font_quad(self.vao, self.vbo, x, y, w, h);

                unsafe {
                    gl::BindVertexArray(self.vao);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, gpu.texture_id);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                }
            }

            // For whitespace, we only update the advance_width.
            advance += character.h_metrics.advance_width;
        });
    }
}
