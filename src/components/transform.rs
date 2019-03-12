use nalgebra_glm as glm;
use serde::Deserialize;
use std::default::Default;

type Vector3 = glm::TVec3<f32>;

#[derive(Debug, Clone, Deserialize)]
pub struct Transform {
    pub position: Vector3,
    pub rotate: Vector3,
    pub scale: Vector3,
}

impl Transform {
    #[allow(unused)]
    pub fn new(position: Vector3, rotate: Vector3, scale: Vector3) -> Self {
        Self {
            position,
            rotate,
            scale,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        let init = glm::vec3(0., 0., 0.);

        Self {
            position: init,
            rotate: init,
            scale: glm::vec3(1., 1., 1.),
        }
    }
}
