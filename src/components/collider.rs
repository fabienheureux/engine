use nphysics3d::{
    object::{ColliderDesc, ColliderHandle},
    material::{MaterialHandle, BasicMaterial},
    world::World,
};

use nalgebra_glm as glm;
use ncollide3d::shape::ShapeHandle;

#[derive(Debug)]
pub struct Collider {
    handle: ColliderHandle,
}

impl Collider {
    pub fn new(
        mut world: &mut World<f32>,
        shape: ShapeHandle<f32>,
        transform: glm::TVec3<f32>,
        density: f32,
    ) -> Self {
        let handle = ColliderDesc::new(shape)
            .translation(transform)
            .material(MaterialHandle::new(BasicMaterial::new(0.3, 0.)))
            .density(density)
            .build(&mut world)
            .handle();

        Self { handle }
    }

    pub fn simple(
        shape: ShapeHandle<f32>,
        transform: glm::TVec3<f32>,
    ) -> ColliderDesc<f32> {
        ColliderDesc::new(shape).translation(transform).density(1.)
    }
}
