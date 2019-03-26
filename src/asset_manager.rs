use crate::{constants::TEXTURE_PATH, opengl::OpenGL, shader::Shader};
use image;
use image::GenericImageView;
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

/// Index of an asset in the storage.
type Indice = usize;

/// This is used by opengl.
/// If None, this asset is currently not yet send into the GPU.
type GlId = Option<u32>;

/// This trait is a "super type" over possible
/// storage types.
trait Ressource {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Default)]
pub struct AssetStorage {
    data: Vec<Box<Ressource>>,
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
            dbg!(texture_path);
            let (width, height) = texture.dimensions();

            let indice = self.storage.data.len();

            let texture = Box::new(Texture {
                raw: texture.to_rgba().into_raw(),
                width: width as i32,
                height: height as i32,
            });

            self.storage.data.insert(indice, texture);

            self.assets.insert(key.clone(), Asset::new(indice, None));
        }

        key
    }

    pub fn add_shader(&mut self, name: &str, vert: &str, frag: &str) -> String {
        let key = String::from(name);

        if !self.assets.contains_key(name) {
            let shader =
                Box::new(Shader::new().with_vert(vert).with_frag(frag));
            let indice = self.storage.data.len();

            self.storage.data.insert(indice, shader);
            self.assets.insert(key.clone(), Asset::new(indice, None));
        }

        key
    }

    pub fn get_mut_asset(&mut self, name: &str) -> &mut Asset {
        self.assets.get_mut(name).expect("Asset not found.")
    }

    pub fn get_asset(&self, name: &str) -> &Asset {
        self.assets.get(name).expect("Asset not found.")
    }

    pub fn get_ressources<T: 'static>(&self) -> Vec<&T> {
        self.storage
            .data
            .iter()
            .filter(|d| d.as_any().is::<T>())
            .map(|d| {
                d.as_any()
                    .downcast_ref::<T>()
                    .expect("Ressource for this type not found.")
            })
            .collect::<Vec<&T>>()
    }

    pub fn get_ressource<T: 'static + Any>(&self, name: &str) -> &T {
        let asset = self.get_asset(name);
        self.storage.data[asset.indice]
            .as_any()
            .downcast_ref::<T>()
            .expect("Ressource for this type not found.")
    }

    #[allow(unused)]
    pub fn remove(&mut self, name: &str) {
        let asset = self.assets.get(name).expect("Asset not found.");

        self.storage.data.remove(asset.indice);

        self.assets
            .remove(name)
            .expect("Error when removing texture from AssetManager");
    }

    /// Send the texture into the renderer.
    pub fn gl_load(&mut self, name: &str) {
        let asset = self.get_asset(name);
        let texture = self.get_ressource::<Texture>(name);

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

#[derive(Debug)]
pub struct Asset {
    pub indice: Indice,
    pub gl_id: GlId,
}

impl Asset {
    pub fn new(indice: Indice, gl_id: GlId) -> Self {
        Self { indice, gl_id }
    }
}

// All images are converted in rgba.
#[derive(Debug)]
pub struct Texture {
    pub raw: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

impl Ressource for Texture {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Ressource for Shader {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
