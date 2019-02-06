use crate::{
    primitives::{Cube, LightSource, Plane},
    world::Entity,
};
use nalgebra_glm as glm;
use ron::de;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Model {
    items: Vec<Elements>,
}

#[derive(Debug, Deserialize)]
enum Elements {
    Cube(f32, f32, f32),
    LightSource(f32, f32, f32),
    Plane(f32, f32, f32),
}

pub fn load_entities() -> Vec<Entity> {
    let mut entities: Vec<Entity> = vec![];
    let file = File::open("assets/ressources/entities.ron")
        .expect("Crash when openning the entities file");

    let model: Model =
        de::from_reader(&file).expect("Crash when deserializing entities");

    model.items.iter().for_each(|item| match *item {
        Elements::Cube(x, y, z) => {
            entities.push(Box::new(Cube::new(glm::vec3(x, y, z))))
        }
        Elements::LightSource(x, y, z) => {
            entities.push(Box::new(LightSource::new(glm::vec3(x, y, z))))
        }
        Elements::Plane(x, y, z) => {
            entities.push(Box::new(Plane::new(glm::vec3(x, y, z))))
        }
    });

    entities
}
