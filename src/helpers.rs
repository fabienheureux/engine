use crate::{opengl::OpenGL, world::Entity};
use nalgebra_glm as glm;
use ron::de;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Model {
    positions: Vec<(f32, f32, f32)>,
}

pub fn load_entities() -> Vec<Entity> {
    let mut entities = vec![];
    let file = File::open("assets/ressources/entities.ron")
        .expect("Crash when openning the entities file");

    let model: Model =
        de::from_reader(&file).expect("Crash when deserializing entities");

    model.positions.iter().for_each(|pos| {
        let &(x, y, z) = pos;
        entities.push(OpenGL::gen_plane(glm::vec3(x, y, z)));
    });

    entities
}
