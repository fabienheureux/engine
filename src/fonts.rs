use nalgebra_glm as glm;
use rusttype::{point, FontCollection, Scale, PositionedGlyph};

pub struct Character {
    texture_id: u32,
    size: glm::TVec2<i32>,
    bearing: glm::TVec2<i32>,
    advance: i32,
}

pub struct Characters {
    characters: Vec<Character>,
}

impl Characters {
    pub fn new() -> Self {
        let ttf = include_bytes!("../assets/fonts/Merriweather-Regular.ttf");
        let collection = FontCollection::from_bytes(ttf as &[u8])
            .expect("Error when creating a font collection.");

        let font = collection
            .into_font()
            .expect("Error when turning font collection into single font.");

        // Desired font pixel height
        let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
        let pixel_height = height.ceil() as usize;

        // 2x scale in x direction to counter the aspect ratio of monospace characters.
        let scale = Scale {
            x: height * 2.0,
            y: height,
        };

        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<PositionedGlyph> =
            font.layout("Engine", scale, offset).collect();

        // Find the most visually pleasing width to display
        let width = glyphs
            .iter()
            .rev()
            .map(|g| {
                g.position().x as f32
                    + g.unpositioned().h_metrics().advance_width
            })
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        println!("width: {}, height: {}", width, pixel_height);

        Self { characters: vec![] }
    }
}
