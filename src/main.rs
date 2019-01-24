mod camera;
mod constants;
mod game_loop;
mod opengl;
mod render;
mod shader;
mod texture;
mod time;
mod window;

use crate::{
    camera::Camera, game_loop::GameLoop, render::Render, texture::Texture,
    window::Window,
};

fn main() {
    let mut window = Window::new();
    let render = Render::new();

    let mut game_loop = GameLoop::new();

    let mut camera = Camera::new();

    let mut t = Texture::new("./assets/textures/wall.jpg");
    t.generate_texture();

    game_loop.start(|time| {
        window.capture();
        let running = !window.should_close;
        let mouse_event = window.get_mouse_events();

        if mouse_event.has_moved {
            camera.update(&mouse_event, &time);
        }

        // Render frame.
        render.draw(&time, t.texture_id, &window, &mut camera);
        window.clean();

        return running;
    });
}
