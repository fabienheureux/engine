use crate::time::Time;
use crate::window::{KeyEvents, Window};
use glutin::{MouseButton, VirtualKeyCode};
use nalgebra_glm as glm;

pub struct Camera {
    pub pos: (f64, f64),
    pub speed: f64,
    pub position: glm::TVec3<f32>,
    pub target: glm::TVec3<f32>,
    pub front: glm::TVec3<f32>,
    pub up: glm::TVec3<f32>,

    pub pitch: f64,
    pub yaw: f64,
    pub first_mouse: bool,
    pub last_pos: (f64, f64),
}

impl Camera {
    pub fn new() -> Self {
        Self {
            speed: 2.5,
            pos: (0., 0.),
            position: glm::vec3(0., 0., 3.),
            front: glm::vec3(0., 0., -1.),
            target: glm::vec3(0., 0., 0.),
            up: glm::vec3(0., 1., 0.),

            first_mouse: true,
            pitch: 0.,
            yaw: -90.,
            last_pos: (0., 0.),
        }
    }

    // Update the camera spin and position from freshly new events.
    pub fn update(&mut self, window: &Window, time: &Time) {
        let mouse = window.get_mouse_events();

        // We want to hide the cursor when we move our camera.
        mouse.trigger_on_press(MouseButton::Right, || {
            window.hide_cursor(true);

            if mouse.has_moved {
                self.update_spin(window, time);
            }
        });

        mouse.trigger_on_release(MouseButton::Right, || {
            window.hide_cursor(false);
        });

        self.update_pos(&window.get_keyboard_events(), time);
    }

    pub fn update_pos(&mut self, keyboard: &KeyEvents, time: &Time) {
        let speed = (self.speed * time.dt) as f32;

        keyboard.trigger_on_press(VirtualKeyCode::W, || {
            self.position += speed * self.front;
        });

        keyboard.trigger_on_press(VirtualKeyCode::S, || {
            self.position -= speed * self.front;
        });

        keyboard.trigger_on_press(VirtualKeyCode::D, || {
            self.position +=
                glm::normalize(&self.front.cross(&self.up)) * speed;
        });

        keyboard.trigger_on_press(VirtualKeyCode::A, || {
            self.position -=
                glm::normalize(&self.front.cross(&self.up)) * speed;
        });

        keyboard.trigger_on_press(VirtualKeyCode::Q, || {
            self.position -= speed * self.up;
        });

        keyboard.trigger_on_press(VirtualKeyCode::E, || {
            self.position += speed * self.up;
        });
    }

    // We are using the delta time for smoother spin.
    pub fn update_spin(&mut self, window: &Window, time: &Time) {
        let mouse_event = window.get_mouse_events();

        let (delta_x, delta_y) = mouse_event.delta;
        self.pos.0 += delta_x;
        self.pos.1 += delta_y;

        let (pos_x, pos_y) = self.pos;

        if self.first_mouse {
            self.last_pos = (pos_x, pos_y);
            self.first_mouse = false;
        }

        let mut x_offset = pos_x - self.last_pos.0;
        let mut y_offset = self.last_pos.1 - pos_y;
        self.last_pos = (pos_x, pos_y);

        x_offset *= self.speed * time.dt;
        y_offset *= self.speed * time.dt;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if self.pitch > 89. {
            self.pitch = 89.;
        }
        if self.pitch < -89. {
            self.pitch = -89.;
        }

        let mut front = glm::vec3(0., 0., 0.);
        let yaw = self.yaw as f32;
        let pitch = self.pitch as f32;

        front.x = yaw.to_radians().cos() * pitch.to_radians().cos();
        front.y = pitch.to_radians().sin();
        front.z = yaw.to_radians().sin() * pitch.to_radians().cos();

        self.front = glm::normalize(&front);
    }
}
