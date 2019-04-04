use crate::GameState;
use crate::{
    components::{RigidBody, Transform},
    ecs::{Entity, System, World},
};
use std::any::TypeId;

#[derive(Debug, Default)]
pub struct Physic;

impl System for Physic {
    fn get_targets(&self) -> Vec<TypeId> {
        vec![
            World::get_type::<Transform>(),
            World::get_type::<RigidBody>(),
        ]
    }

    fn process(&self, entity: &mut Entity, state: &mut GameState) {
        let rigid = entity.get::<RigidBody>();

        let body = rigid.get(&state.physic_world);

        if let Some(part) = body.part(0) {
            let position = part.position();

            let transform = entity.get_mut::<Transform>();
            transform.rotation = position.rotation;
            transform.position = position.translation.vector;
        }
    }
}
