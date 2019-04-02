use nalgebra_glm as glm;
use nphysics3d::{
    object::{Body, BodyHandle, ColliderDesc, RigidBodyDesc},
    world::World,
};

#[derive(Debug)]
pub struct RigidBody {
    mass: f32,
    handle: BodyHandle,
}

impl RigidBody {
    pub fn new(
        mut world: &mut World<f32>,
        mass: f32,
        transform: glm::TVec3<f32>,
        collider: Option<ColliderDesc<f32>>,
    ) -> Self {
        let mut body = RigidBodyDesc::new().mass(mass).translation(transform);

        if let Some(collider) = &collider {
            body.add_collider(collider);
        };

        // Register this body in the physic world.
        let b = body.build(&mut world);
        // Get the handle to retrieve this body later.
        let handle = b.handle();

        Self { mass, handle }
    }

    pub fn get<'a>(&self, world: &'a World<f32>) -> &'a Body<f32> {
        world
            .body(self.handle)
            .expect("Handle not register in physic world")
    }
}
