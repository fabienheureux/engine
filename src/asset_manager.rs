use crate::{constants::TEXTURE_PATH, opengl::OpenGL, shader::Shader};
use image;
use image::GenericImageView;
use std::collections::HashMap;
use std::path::Path;

// Index of the asset in the storage Vec.
type Indice = usize;
// This is used by opengl.
// If None, this asset is currently not loaded into the renderer.
type GlId = Option<u32>;

// Each type has his own Vec storage.
#[derive(Debug)]
pub enum StorageType {
    Texture,
    // Audio, Mesh, ...
}

#[derive(Debug)]
pub struct Asset {
    pub indice: Indice,
    pub kind: StorageType,
    pub gl_id: GlId,
}

// All image are converted in rgba.
#[derive(Debug)]
pub struct Image {
    raw: Vec<u8>,
    width: i32,
    height: i32,
}

impl Asset {
    pub fn new(indice: Indice, kind: StorageType, gl_id: GlId) -> Self {
        Self {
            indice,
            kind,
            gl_id,
        }
    }
}

#[derive(Default)]
pub struct AssetStorage {
    pub textures: Vec<Image>,
    pub shaders: Vec<Shader>,
}

#[derive(Default)]
pub struct AssetManager {
    storage: AssetStorage,
    assets: HashMap<String, Asset>,
}

impl AssetManager {
    pub fn add_texture(&mut self, path: &str) -> String {
        let key = String::from(path);

        if !self.assets.contains_key(path) {
            let texture_path = String::from([TEXTURE_PATH, path].join(""));
            let texture = Self::memory_load(texture_path.as_str());
            let (width, height) = texture.dimensions();

            let indice = self.storage.textures.len();
            self.storage.textures.insert(indice, Image{
                raw: texture.to_rgba().into_raw(),
                width: width as i32, 
                height: height as i32, 
            });

            self.assets.insert(
                key.clone(),
                Asset::new(indice, StorageType::Texture, None),
            );
        }

        return key;
    }

    pub fn get_mut_asset(&mut self, name: &str) -> &mut Asset {
        self.assets.get_mut(name).expect("Asset not found.")
    }

    pub fn get_asset(&self, name: &str) -> &Asset {
        self.assets.get(name).expect("Asset not found.")
    }

    #[allow(unused)]
    pub fn remove_texture(&mut self, name: &str) {
        let asset = self.assets.get(name).expect("Asset not found.");

        self.storage.textures.remove(asset.indice);

        self.assets
            .remove(name)
            .expect("Error when removing texture from AssetManager");
    }

    /// This method can load the attached texture into the memory and give it
    /// to the GPU.
    /// For now, there are some openGL config stuff.
    pub fn gl_load(&mut self, name: &str) {
        let asset = self.get_asset(name);

        // We want to sent the texture only once into the GPU.
        if asset.gl_id.is_some() {
            return;
        }

        let indice = asset.indice;

        let image = self
            .storage
            .textures
            .get(indice)
            .expect("Texture not found in storage.");

        let id = OpenGL::load_2d_texture(
            image.width,
            image.height, 
            image.raw.clone(),
        );

        // Bind the gl id to the asset.
        let asset = self.get_mut_asset(name);
        asset.gl_id = Some(id);
    }

    /// Load the image into the memory.
    fn memory_load(path: &str) -> image::DynamicImage {
        let path = Path::new(path);
        image::open(path).expect("Failed to load image")
    }
}
