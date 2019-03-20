use crate::{
    components::{Camera, Light, Mesh, Primitives, Transform},
    constants::SCENE_PATH,
    ecs::Entity,
    game_state::GameState,
};
use ron::de;
use serde::Deserialize;
use std::fs::File;

static mut IS_FIRST_LOAD: bool = true;

#[derive(Deserialize)]
struct Model {
    items: Vec<Elements>,
}

#[derive(Deserialize)]
enum Elements {
    Camera(usize, Transform),
    Cube(usize, Transform, String),
    Plane(usize, Transform, String),
    LightSource(usize, Transform, Light),
}

pub fn load_scene(path: &str, state: &mut GameState) -> Vec<Entity> {
    let asset_manager = &mut state.asset_manager;
    let mut entities: Vec<Entity> = vec![];

    let path = [SCENE_PATH, path].join("");
    let file = File::open(path).expect("Crash when openning the entities file");

    let model: Model =
        de::from_reader(&file).expect("Crash when deserializing entities");

    for item in model.items.into_iter() {
        match item {
            Elements::LightSource(id, transform, mut light) => {
                let mesh = Mesh::new(Primitives::Cube, None, "light");
                light.set_ubo();

                let entity = Entity::from_file(id)
                    .with::<Transform>(transform)
                    .with::<Light>(light)
                    .with::<Mesh>(mesh);

                entities.push(entity);
            }
            Elements::Cube(id, transform, texture) => {
                let texture = texture.as_str();
                let mut opt_tex = None;

                if !texture.is_empty() {
                    opt_tex = Some(asset_manager.add_texture(texture));
                    asset_manager.gl_load(texture);
                }

                let mesh =
                    Mesh::new(Primitives::Cube, opt_tex, "default_material");

                let entity = Entity::from_file(id)
                    .with::<Transform>(transform)
                    .with::<Mesh>(mesh);

                entities.push(entity);
            }
            Elements::Plane(id, transform, texture) => {
                let texture = texture.as_str();
                let mut opt_tex = None;

                if !texture.is_empty() {
                    opt_tex = Some(asset_manager.add_texture(texture));
                    asset_manager.gl_load(texture);
                }

                let mesh =
                    Mesh::new(Primitives::Plane, opt_tex, "default_material");

                let entity = Entity::from_file(id)
                    .with::<Transform>(transform)
                    .with::<Mesh>(mesh);

                entities.push(entity);
            }
            Elements::Camera(id, transform) => {
                if unsafe { IS_FIRST_LOAD } {
                    let entity = Entity::from_file(id)
                        .with::<Transform>(transform)
                        .with::<Camera>(Camera::default());

                    entities.push(entity);
                }
            }
        }
    }

    unsafe {
        if IS_FIRST_LOAD {
            IS_FIRST_LOAD = false
        }
    }

    entities
}
