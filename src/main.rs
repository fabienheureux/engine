mod camera;
mod constants;
mod game_loop;
mod opengl;
mod shader;
mod texture;
mod time;
mod window;
mod world;

use crate::{
    camera::Camera, game_loop::GameLoop, opengl::OpenGL, window::Window,
    world::World,
};
use nalgebra_glm as glm;

fn main() {
    let mut window = Window::new();
    let mut game_loop = GameLoop::new();
    let mut camera = Camera::new();

    let mut world = World::new()
        .with_entity(OpenGL::gen_plane(glm::vec3(0., 0., 1.)))
        .with_entity(OpenGL::gen_plane(glm::vec3(0., 0., 0.)));

    game_loop.start(|time| {
        window.capture();
        let running = !window.should_close;

        camera.update(&window, &time);

        // Render frame.
        world.draw(&window, &mut camera);
        running
    });
}
