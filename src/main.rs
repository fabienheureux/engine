mod game_controller;
mod game_loop;
mod opengl;
mod render;
mod shader;
mod texture;
mod time;
mod window;

use crate::{
    game_controller::{GameController, Input},
    game_loop::GameLoop,
    render::Render,
    texture::Texture,
    window::Window,
};

const GAME_TITLE: &str = "Neo Pac-Man";

fn main() {
    let window = Window::new(GAME_TITLE);
    let render = Render::new(&window.gl_window);

    let mut game_controller = GameController::new(window.event_loop);
    let mut game_loop = GameLoop::new();

    let mut t = Texture::new("assets/textures/wall.jpg");
    t.generate_texture();

    game_loop.start(|time| {
        // Process inputs.
        let mut running = true;
        let input = game_controller.pull();

        if input.is_some() {
            running = input.unwrap() != Input::CloseRequested;
        }

        // Render frame.
        render.draw(&time, t.texture_id);

        running
    });
}
