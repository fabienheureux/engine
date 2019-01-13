mod game_controller;
mod game_loop;
mod render;
mod shader;
mod time;

use crate::{
    game_controller::{GameController, Input},
    game_loop::GameLoop,
    render::Render,
};
use gl;
use glutin::{ContextBuilder, EventsLoop, GlContext, GlWindow, WindowBuilder};

const GAME_TITLE: &str = "Neo Pac-Man";

fn main() {
    let window = WindowBuilder::new().with_title(GAME_TITLE);
    let context = ContextBuilder::new();
    let event_loop = EventsLoop::new();
    let gl_window = GlWindow::new(window, context, &event_loop)
        .expect("Error creating opengl window");

    unsafe {
        gl_window
            .make_current()
            .expect("Error setting current context")
    }

    let render = Render::new(gl_window);
    let mut game_controller = GameController::new(event_loop);
    let mut game_loop = GameLoop::new();

    game_loop.start(|time| {
        // Process inputs.
        let mut running = true;
        let input = game_controller.pull();

        if input.is_some() {
            running = input.unwrap() != Input::CloseRequested;
        }

        // Render frame.
        render.draw(&time);

        running
    });
}
