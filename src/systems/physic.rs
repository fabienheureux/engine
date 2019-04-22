use crate::GameState;
use crate::{
    components::{RigidBody, Transform},
    ecs::{Entity, System, World},
};
use glutin::VirtualKeyCode;
use nalgebra_glm as glm;
use nphysics3d::math::{Force, ForceType};
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

        let body = rigid.get_mut_body(&mut state.physic_world);

        let keyboard = state.window.get_keyboard_events();
        keyboard.once(VirtualKeyCode::Space, || {
            body.apply_force(
                0,
                &Force::linear(glm::vec3(0., 10., 0.)),
                ForceType::Impulse,
                true,
            );
        });

        if let Some(part) = body.part(0) {
            let position = part.position();

            let transform = entity.get_mut::<Transform>();
            transform.rotation = position.rotation;
            transform.position = position.translation.vector;
        }
    }
}
