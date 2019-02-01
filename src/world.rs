use crate::camera::Camera;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::shader::Shader;
use crate::window::Window;
use gl;
use nalgebra_glm as glm;

pub type Entity = Box<dyn Renderer>;

pub trait Renderer: std::fmt::Debug {
    fn draw(&self, model: glm::Mat4);
}

#[derive(Debug)]
pub struct World {
    pub entities: Vec<Entity>,
    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,
    shader: Shader,
}

impl World {
    pub fn new() -> Self {
        let shader = Shader::new("shaders", "default_cube");

        Self {
            entities: vec![],
            model: glm::rotate_x(
                &glm::Mat4::identity(),
                -(55_f32.to_radians()),
            ),
            view: glm::Mat4::identity(),
            projection: glm::perspective(
                45_f32.to_radians(),
                (SCREEN_WIDTH / SCREEN_HEIGHT) as f32,
                0.1,
                100.,
            ),
            shader,
        }
    }

    #[allow(unused)]
    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn draw(&mut self, window: &Window, cam: &mut Camera) {
        self.view =
            glm::look_at(&cam.position, &(cam.position + cam.front), &cam.up);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            // Clear color buffer with the color specified by gl::ClearColor.
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(self.shader.id);
        }

        self.shader.set_matrix4("view", glm::value_ptr(&self.view));
        self.shader
            .set_matrix4("projection", glm::value_ptr(&self.projection));

        self.entities.as_slice().iter().for_each(|entity| {
            entity.draw(self.model);
        });

        // Cleanup
        window
            .gl_window
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }
}
