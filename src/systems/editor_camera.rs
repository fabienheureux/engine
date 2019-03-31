use crate::window::KeyEvents;
use crate::GameState;
use crate::{
    components::{Camera, Transform},
    ecs::{Entity, System, World},
    opengl::OpenGL,
    shader::Shader,
    time::Time,
    window::Window,
};
use glutin::{MouseButton, VirtualKeyCode};
use nalgebra_glm as glm;
use std::any::TypeId;

#[derive(Debug, Default)]
pub struct EditorCamera;

impl System for EditorCamera {
    fn get_targets(&self) -> Vec<TypeId> {
        vec![World::get_type::<Transform>(), World::get_type::<Camera>()]
    }

    fn process(&self, entity: &mut Entity, state: &mut GameState) {
        if !state.editor_mode {
            return;
        }

        let mut window = &state.window;
        let time = &state.time;
        let mouse = window.get_mouse_events();

        // We want to hide the cursor when we move our camera.
        mouse.trigger_on_press(MouseButton::Right, || {
            window.hide_cursor(true);

            if mouse.has_moved {
                update_spin(entity, &mut window, &time);
            }
        });

        mouse.trigger_on_release(MouseButton::Right, || {
            window.hide_cursor(false);
        });

        update_pos(entity, &window.get_keyboard_events(), &time);

        let cam = entity.get::<Camera>();
        let transform = entity.get::<Transform>();

        let cam_pos = format!(
            "({:.1},{:.1},{:.1})",
            transform.position.x,
            transform.position.y,
            transform.position.z
        );
        state.cam_pos = cam_pos;

        let view = glm::look_at(
            &transform.position,
            &(transform.position + cam.front),
            &cam.up,
        );

        OpenGL::set_mat4_to_ubo(view, state.camera_ubo, 64);
        OpenGL::set_vec3_to_ubo(transform.position, state.camera_ubo, 192);

        let center = glm::vec3(0., 0., 0.);
        let sk = glm::look_at(&center, &(center + cam.front), &cam.up);
        OpenGL::set_mat4_to_ubo(sk, state.camera_ubo, 128);
    }
}

fn update_pos(entity: &mut Entity, keyboard: &KeyEvents, time: &Time) {
    let (speed, front, up) = {
        let cam = entity.get::<Camera>();
        (cam.speed, cam.front, cam.up)
    };
    let transform = entity.get_mut::<Transform>();

    let mut speed = (speed * time.dt) as f32;

    if keyboard.modifiers.shift {
        speed *= 2.5;
    }

    keyboard.trigger_on_press(VirtualKeyCode::W, || {
        transform.position += speed * front;
    });

    keyboard.trigger_on_press(VirtualKeyCode::S, || {
        transform.position -= speed * front;
    });

    keyboard.trigger_on_press(VirtualKeyCode::D, || {
        transform.position += glm::normalize(&front.cross(&up)) * speed;
    });

    keyboard.trigger_on_press(VirtualKeyCode::A, || {
        transform.position -= glm::normalize(&front.cross(&up)) * speed;
    });

    keyboard.trigger_on_press(VirtualKeyCode::Q, || {
        transform.position -= speed * up;
    });

    keyboard.trigger_on_press(VirtualKeyCode::E, || {
        transform.position += speed * up;
    });
}

// We are using the delta time for smoother spin.
fn update_spin(entity: &mut Entity, window: &Window, time: &Time) {
    let cam = entity.get_mut::<Camera>();
    let mouse_event = window.get_mouse_events();

    let (delta_x, delta_y) = mouse_event.delta;
    cam.pos.0 += delta_x;
    cam.pos.1 += delta_y;

    let (pos_x, pos_y) = cam.pos;

    if cam.first_mouse {
        cam.last_pos = (pos_x, pos_y);
        cam.first_mouse = false;
    }

    let mut x_offset = pos_x - cam.last_pos.0;
    let mut y_offset = cam.last_pos.1 - pos_y;
    cam.last_pos = (pos_x, pos_y);

    x_offset *= cam.speed * time.dt;
    y_offset *= cam.speed * time.dt;

    cam.yaw += x_offset;
    cam.pitch += y_offset;

    if cam.pitch > 89. {
        cam.pitch = 89.;
    }
    if cam.pitch < -89. {
        cam.pitch = -89.;
    }

    let mut front = glm::vec3(0., 0., 0.);
    let yaw = cam.yaw as f32;
    let pitch = cam.pitch as f32;

    front.x = yaw.to_radians().cos() * pitch.to_radians().cos();
    front.y = pitch.to_radians().sin();
    front.z = yaw.to_radians().sin() * pitch.to_radians().cos();

    cam.front = glm::normalize(&front);
}
