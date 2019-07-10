use crate::{
    components::{
        Camera, Collider, Light, Mesh, Player, Primitives, RigidBody, Transform,
    },
    constants::SCENE_PATH,
    ecs::Entity,
    ecs::World,
    game_state::GameState,
};
use nalgebra_glm as glm;
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::object::BodyStatus;
use ron::de;
use serde::Deserialize;
use std::fs::File;

use notify::{
    DebouncedEvent, RecommendedWatcher, RecursiveMode,
    Watcher,
};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

static mut IS_FIRST_LOAD: bool = true;

#[derive(Deserialize)]
struct Body {
    mass: f32,
}

#[derive(Deserialize)]
struct Model {
    items: Vec<Elements>,
}

#[derive(Deserialize)]
enum Elements {
    Camera(usize, Transform),
    Cube(usize, Transform, String, Body),
    Plane(usize, Transform, String),
    LightSource(usize, Transform, Light),
    Player(usize, Transform, String, Body),
}

/// A scene loader.
pub struct SceneLoader {
    current: Option<String>,
    receiver: Receiver<DebouncedEvent>,
    #[allow(unused)]
    watcher: RecommendedWatcher,
}

impl SceneLoader {
    pub fn new(delay: u64) -> Self {
        let (sender, receiver) = channel();
        let mut watcher: RecommendedWatcher =
            Watcher::new(sender, Duration::from_secs(delay)).unwrap();

        watcher
            .watch(SCENE_PATH, RecursiveMode::NonRecursive)
            .unwrap();

        Self {
            watcher,
            receiver,
            current: None,
        }
    }

    pub fn set_scene(&mut self, path: &str) {
        self.current = Some(String::from(path));
    }

    pub fn watch(&self, world: &mut World, state: &mut GameState) {
        if self.receiver.try_recv().is_ok() {
            self.load(world, state);
        }
    }

    pub fn load(&self, world: &mut World, mut state: &mut GameState) {
        if let Some(scene_path) = self.current.as_ref() {
            world.load_entities(Self::load_scene(scene_path, &mut state));
        }
    }

    pub fn load_scene(scene: &str, state: &mut GameState) -> Vec<Entity> {
        let asset_manager = &mut state.asset_manager;
        let mut entities: Vec<Entity> = vec![];

        let path = [SCENE_PATH, scene].join("");
        let file =
            File::open(path).expect("Crash when openning the entities file");

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
                Elements::Cube(id, transform, texture, body) => {
                    let texture = texture.as_str();
                    let mut opt_tex = None;

                    if !texture.is_empty() {
                        opt_tex = Some(asset_manager.add_texture(texture));
                        asset_manager.gl_load(texture);
                    }

                    let mesh = Mesh::new(
                        Primitives::Cube,
                        opt_tex,
                        "default_material",
                    );

                    let shape =
                        ShapeHandle::new(Cuboid::new(glm::vec3(0.5, 0.5, 0.5)));
                    let collider =
                        Collider::simple(shape, glm::vec3(0., 0., 0.));

                    let rigid_body = RigidBody::new(
                        &mut state.physic_world,
                        body.mass,
                        transform.position,
                        BodyStatus::Dynamic,
                        Some(collider),
                    );

                    let entity = Entity::from_file(id)
                        .with::<Transform>(transform)
                        .with::<RigidBody>(rigid_body)
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

                    let mesh = Mesh::new(
                        Primitives::Plane,
                        opt_tex,
                        "default_material",
                    );

                    let shape =
                        ShapeHandle::new(Cuboid::new(glm::vec3(5., 0.04, 5.)));
                    let collider = Collider::new(
                        &mut state.physic_world,
                        shape,
                        transform.position,
                        1.,
                    );

                    let entity = Entity::from_file(id)
                        .with::<Collider>(collider)
                        .with::<Transform>(transform)
                        .with::<Mesh>(mesh);

                    entities.push(entity);
                }
                Elements::Player(id, transform, texture, body) => {
                    let texture = texture.as_str();
                    let mut opt_tex = None;

                    if !texture.is_empty() {
                        opt_tex = Some(asset_manager.add_texture(texture));
                        asset_manager.gl_load(texture);
                    }

                    let mesh = Mesh::new(
                        Primitives::Cube,
                        opt_tex,
                        "default_material",
                    );

                    let shape =
                        ShapeHandle::new(Cuboid::new(glm::vec3(0.5, 0.5, 0.5)));
                    let collider =
                        Collider::simple(shape, glm::vec3(0., 0., 0.));

                    let rigid_body = RigidBody::new(
                        &mut state.physic_world,
                        body.mass,
                        transform.position,
                        BodyStatus::Kinematic,
                        Some(collider),
                    );

                    let entity = Entity::from_file(id)
                        .with::<Transform>(transform)
                        .with::<RigidBody>(rigid_body)
                        .with::<Player>(Player::default())
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
}
