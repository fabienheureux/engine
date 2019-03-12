use crate::{components::Transform, opengl::OpenGL};
use nalgebra_glm as glm;
use serde::Deserialize;
use std::mem;

static mut UBO_LIGHT_INDEX: usize = 0;

type Vector3 = glm::TVec3<f32>;

// 1 -> Sun light
// 2 -> Directional light
// 3 -> Point light
// 4 -> Spot light
// let l_type = 1;
#[derive(Debug, Deserialize)]
pub enum Lights {
    Sun,
    Directional,
    Point,
    Spotlight,
}

#[derive(Debug, Deserialize)]
pub struct Light {
    ubo_index: usize,
    pub kind: Lights,
    pub direction: Vector3,
    pub ambient: Vector3,
    pub diffuse: Vector3,
    pub specular: Vector3,
}

impl Light {
    #[allow(unused)]
    pub fn new(
        kind: Lights,
        direction: Vector3,
        ambient: Vector3,
        diffuse: Vector3,
        specular: Vector3,
    ) -> Self {
        unsafe {
            if UBO_LIGHT_INDEX > 0 {
                UBO_LIGHT_INDEX += 1;
            }

            Self {
                ubo_index: UBO_LIGHT_INDEX,
                kind,
                direction,
                ambient,
                diffuse,
                specular,
            }
        }
    }

    pub fn set_ubo(&mut self) {
        unsafe {
            self.ubo_index = UBO_LIGHT_INDEX;
            UBO_LIGHT_INDEX += 1;
        }
    }

    pub fn set_to_shader(&self, lights_ubo: u32, transform: &Transform) {
        let size = mem::size_of::<glm::TVec4<f32>>() as isize;
        let block_offset = 96 * self.ubo_index as isize;

        let kind = match self.kind {
            Lights::Sun => 1,
            Lights::Directional => 2,
            _ => 1,
        };

        OpenGL::set_int_to_ubo(kind, lights_ubo, block_offset);
        OpenGL::set_vec3_to_ubo(
            self.direction,
            lights_ubo,
            size + block_offset,
        );
        OpenGL::set_vec3_to_ubo(
            transform.position,
            lights_ubo,
            2 * size + block_offset,
        );
        OpenGL::set_vec3_to_ubo(
            self.ambient,
            lights_ubo,
            3 * size + block_offset,
        );
        OpenGL::set_vec3_to_ubo(
            self.diffuse,
            lights_ubo,
            4 * size + block_offset,
        );
        OpenGL::set_vec3_to_ubo(
            self.specular,
            lights_ubo,
            5 * size + block_offset,
        );
    }
}
