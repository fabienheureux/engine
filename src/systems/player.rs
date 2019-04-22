use crate::GameState;
use crate::{
    components::{Player as PlayerComponent, RigidBody, Transform},
    ecs::{Entity, System, World},
};
use glutin::VirtualKeyCode;
use nalgebra as na;
use nalgebra_glm as glm;
use na::{geometry::Translation};
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
        let rigid = entity.get_mut::<RigidBody>();
        let body = rigid.get_mut(&mut state.physic_world);

        let time = &state.time;
        let keyboard = state.window.get_keyboard_events();

        let speed = 0.8 * time.dt as f32;
        let mut vector = glm::vec3(0., 0., 0.);

        // Editor move.
        if keyboard.modifiers.shift {
            return;
        }

        keyboard.pressed(VirtualKeyCode::D, || {
            vector.x = speed
        });

        keyboard.pressed(VirtualKeyCode::A, || {
            vector.x = -speed
        });

        keyboard.pressed(VirtualKeyCode::W, || {
            vector.z = -speed
        });

        keyboard.pressed(VirtualKeyCode::S, || {
            vector.z = speed
        });

        let translation = Translation::from(vector);

        let mut position = body.position().clone();
        position.append_translation_mut(&translation);
        body.set_position(position);
    }
}
