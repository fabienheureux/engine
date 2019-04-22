use nalgebra_glm as glm;
use nphysics3d::{
    math::Velocity,
    object::{
        Body, BodyHandle, BodyStatus, ColliderDesc, RigidBody as NRigidBody,
        RigidBodyDesc,
    },
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
        status: BodyStatus,
        collider: Option<ColliderDesc<f32>>,
    ) -> Self {
        let mut body = RigidBodyDesc::new()
            .mass(mass)
            .translation(transform)
            .status(status);

        if status == BodyStatus::Kinematic {
            body.set_velocity(Velocity::linear(0., 0., 0.));
        }

        if let Some(collider) = &collider {
            body.add_collider(collider);
        };

        // Register this body in the physic world.
        let b = body.build(&mut world);
        // Get the handle to retrieve this body later.
        let handle = b.handle();

        Self { mass, handle }
    }

    pub fn get_body<'a>(&self, world: &'a World<f32>) -> &'a Body<f32> {
        world
            .body(self.handle)
            .expect("Handle not register in physic world")
    }

    pub fn get_mut_body<'a>(
        &self,
        world: &'a mut World<f32>,
    ) -> &'a mut Body<f32> {
        world
            .body_mut(self.handle)
            .expect("Handle not register in physic world")
    }

    pub fn get<'a>(&self, world: &'a World<f32>) -> &'a NRigidBody<f32> {
        world
            .rigid_body(self.handle)
            .expect("Handle not register in physic world")
    }

    pub fn get_mut<'a>(
        &self,
        world: &'a mut World<f32>,
    ) -> &'a mut NRigidBody<f32> {
        world
            .rigid_body_mut(self.handle)
            .expect("Handle not register in physic world")
    }
}
