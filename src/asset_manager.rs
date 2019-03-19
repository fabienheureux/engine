use crate::{constants::TEXTURE_PATH, opengl::OpenGL};
use image;
use image::GenericImageView;
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

/// Index of the asset in the storage.
type Indice = usize;

/// This is used by opengl.
/// If None, this asset is currently not yet send into the GPU.
type GlId = Option<u32>;

/// This enum store is a "super type" over possible
/// storage types.
pub enum Ressource {
    Texture(Texture),
}
impl Ressource {
    pub fn get_raw<T: Any>(&self) -> &T {
        use Ressource::*;

        match self {
            Texture(value) => {
                let any = value as &dyn Any;
                any.downcast_ref::<T>().expect("Not found")
            }
        }
    }

    #[allow(unused)]
    pub fn get_mut_raw<T: Any>(&mut self) -> &mut T {
        use Ressource::*;

        match self {
            Texture(value) => {
                let any = value as &mut dyn Any;
                any.downcast_mut::<T>().expect("Not found")
            }
        }
    }
}

#[derive(Debug)]
pub struct Asset {
    pub indice: Indice,
    pub gl_id: GlId,
}

// All image are converted in rgba.
#[derive(Debug)]
pub struct Texture {
    raw: Vec<u8>,
    width: i32,
    height: i32,
}

impl Asset {
    pub fn new(indice: Indice, gl_id: GlId) -> Self {
        Self { indice, gl_id }
    }
}

#[derive(Default)]
pub struct AssetStorage {
    pub textures: Vec<Ressource>,
    pub shaders: Vec<Ressource>,
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

            let texture = Ressource::Texture(Texture {
                raw: texture.to_rgba().into_raw(),
                width: width as i32,
                height: height as i32,
            });

            self.storage.textures.insert(indice, texture);

            self.assets.insert(key.clone(), Asset::new(indice, None));
        }

        return key;
    }

    pub fn get_mut_asset(&mut self, name: &str) -> &mut Asset {
        self.assets.get_mut(name).expect("Asset not found.")
    }

    pub fn get_asset(&self, name: &str) -> &Asset {
        self.assets.get(name).expect("Asset not found.")
    }

    pub fn get<T: 'static>(&self, name: &str) -> (&Asset, &T) {
        let asset = self.get_asset(name);
        let ressources = self.get_ressource::<T>(asset);

        (asset, ressources)
    }

    pub fn get_ressource<T: 'static>(&self, asset: &Asset) -> &T {
        let indice = asset.indice;

        self.storage
            .textures
            .get(indice)
            .expect("Texture not found in storage.")
            .get_raw::<T>()
    }

    #[allow(unused)]
    pub fn remove_texture(&mut self, name: &str) {
        let asset = self.assets.get(name).expect("Asset not found.");

        self.storage.textures.remove(asset.indice);

        self.assets
            .remove(name)
            .expect("Error when removing texture from AssetManager");
    }

    /// Send the texture into the renderer.
    pub fn gl_load(&mut self, name: &str) {
        let (asset, texture) = self.get::<Texture>(name);

        // We want to sent the texture only once into the GPU.
        if asset.gl_id.is_some() {
            return;
        }

        // This id is used to activate the texture in the renderer system.
        let id = OpenGL::load_2d_texture(
            texture.width,
            texture.height,
            &texture.raw,
        );

        // Bind the id to the asset object, to be retrieved later.
        let asset = self.get_mut_asset(name);
        asset.gl_id = Some(id);
    }

    /// Load the image into the memory.
    fn memory_load(path: &str) -> image::DynamicImage {
        let path = Path::new(path);
        image::open(path).expect("Failed to load image")
    }
}
