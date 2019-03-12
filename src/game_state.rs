use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    opengl::OpenGL,
    time::Time,
    window::Window,
};
use nalgebra_glm as glm;

pub struct GameState {
    pub window: Window,
    pub time: Time,
    pub camera_ubo: u32,
    pub lights_ubo: u32,
    pub editor_mode: bool,
}

impl GameState {
    pub fn new() -> Self {
        let window = Window::new();
        let projection = glm::perspective(
            45_f32.to_radians(),
            (SCREEN_WIDTH / SCREEN_HEIGHT) as f32,
            0.1,
            100.,
        );

        let camera_ubo = OpenGL::create_camera_ubo(0);
        OpenGL::set_mat4_to_ubo(projection, camera_ubo, 0);

        let lights_ubo = OpenGL::create_lights_ubo(1);

        Self {
            window,
            time: Time::default(),
            editor_mode: true,
            camera_ubo,
            lights_ubo,
        }
    }
}
