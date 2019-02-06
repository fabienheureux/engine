use crate::camera::Camera;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::opengl::OpenGL;
use crate::window::Window;
use gl;
use glutin::VirtualKeyCode;
use nalgebra_glm as glm;

pub type Entity = Box<dyn Renderer>;

pub trait Renderer: std::fmt::Debug {
    fn draw(&self, model: glm::Mat4, view: glm::Mat4, proj: glm::Mat4);
}

#[derive(Debug)]
pub struct World {
    pub entities: Vec<Entity>,
    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,
}

impl World {
    pub fn new() -> Self {
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
        }
    }

    #[allow(unused)]
    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn set_polygon_mode(&self, window: &Window) {
        let keyboard = window.get_keyboard_events();

        // Only if ctrl is pressed.
        if !keyboard.modifiers.ctrl {
            return;
        }

        keyboard.trigger_on_press(VirtualKeyCode::L, || {
            OpenGL::line_mode();
        });

        keyboard.trigger_on_press(VirtualKeyCode::F, || {
            OpenGL::fill_mode();
        });
    }

    pub fn draw(&mut self, window: &Window, cam: &mut Camera) {
        self.set_polygon_mode(window);

        self.view =
            glm::look_at(&cam.position, &(cam.position + cam.front), &cam.up);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            // Clear color buffer with the color specified by gl::ClearColor.
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.entities.as_slice().iter().for_each(|entity| {
            entity.draw(self.model, self.view, self.projection);
        });

        // Cleanup
        window
            .gl_window
            .swap_buffers()
            .expect("Problem with gl buffer swap");
    }
}
