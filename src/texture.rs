use gl;
use image;
use std::ffi::c_void;
use std::path::Path;

/// We don't want to keep any texture images in
/// the memory, so we just store the file path.
pub struct Texture<'a> {
    pub texture_id: u32,
    texture_path: &'a str,
}

impl<'a> Texture<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            texture_id: 0,
            texture_path: path,
        }
    }

    /// This method can load the attached texture into the memory and give it
    /// to the GPU.
    /// For now, there are some openGL config stuff.
    pub fn generate_texture(&mut self) {
        let image = Self::load(self.texture_path).to_rgb();
        let width = image.width() as i32;
        let height = image.height() as i32;

        let raw_data = image.into_raw();

        unsafe {
            gl::GenTextures(1, &mut self.texture_id);
            gl::BindTexture(gl::TEXTURE, self.texture_id);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            // Load texture data.
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &raw_data[0] as *const u8 as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    /// Load the image into the memory.
    fn load(path: &str) -> image::DynamicImage {
        let path = Path::new(path);
        image::open(path).expect("Failed to load image")
    }
}
