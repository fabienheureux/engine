use crate::GameState;
use crate::{
    components::{Player as PlayerComponent, RigidBody, Transform},
    ecs::{Entity, System, World},
};
use glutin::VirtualKeyCode;
use nalgebra_glm as glm;
use nphysics3d::math::{Force, ForceType};
use std::any::TypeId;

#[derive(Debug, Default)]
pub struct Player;

impl System for Player {
    fn get_targets(&self) -> Vec<TypeId> {
        vec![
            World::get_type::<Transform>(),
            World::get_type::<RigidBody>(),
            World::get_type::<PlayerComponent>(),
        ]
    }

    fn process(&self, entity: &mut Entity, state: &mut GameState) {
        let rigid = entity.get::<RigidBody>();
        let body = rigid.get_mut(&mut state.physic_world);

        let keyboard = state.window.get_keyboard_events();

        keyboard.pressed(VirtualKeyCode::W, || {
            body.apply_force(
                0,
                &Force::linear(glm::vec3(0., 10., 0.)),
                ForceType::VelocityChange,
                true,
            );
        });
    }
}
