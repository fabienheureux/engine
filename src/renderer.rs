use crate::{time::Time, Camera};
use nalgebra_glm as glm;

pub trait Renderer: std::fmt::Debug {
    fn draw(
        &mut self,
        time: &Time,
        camera: &Camera,
        view: glm::Mat4,
        proj: glm::Mat4,
    );
}
