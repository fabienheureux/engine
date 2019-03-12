use nalgebra_glm as glm;
use serde::Deserialize;
use std::default::Default;

#[derive(Debug, Clone, Deserialize)]
pub struct Camera {
    pub pos: (f64, f64),
    pub speed: f64,
    pub target: glm::TVec3<f32>,
    pub front: glm::TVec3<f32>,
    pub up: glm::TVec3<f32>,

    pub pitch: f64,
    pub yaw: f64,
    pub first_mouse: bool,
    pub last_pos: (f64, f64),
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            speed: 2.5,
            pos: (0., 0.),
            front: glm::vec3(0., 0., -1.),
            target: glm::vec3(0., 0., 0.),
            up: glm::vec3(0., 1., 0.),

            first_mouse: true,
            pitch: 0.,
            yaw: -90.,
            last_pos: (0., 0.),
        }
    }
}
