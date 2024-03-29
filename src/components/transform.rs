use nalgebra_glm as glm;
use serde::Deserialize;
use std::default::Default;

use nalgebra;

type Vector3 = glm::TVec3<f32>;

#[derive(Debug, Clone, Deserialize)]
pub struct Transform {
    pub position: Vector3,
    pub scale: Vector3,
    pub rotation: nalgebra::geometry::UnitQuaternion<f32>,
}

impl Transform {
    #[allow(unused)]
    pub fn new(position: Vector3, rotate: Vector3, scale: Vector3) -> Self {
        Self {
            position,
            scale,
            ..Self::default()
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        let init = glm::vec3(0., 0., 0.);

        Self {
            position: init,
            scale: glm::vec3(1., 1., 1.),
            rotation: nalgebra::UnitQuaternion::identity(),
        }
    }
}
