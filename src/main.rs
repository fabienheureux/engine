mod render;
mod shader;
mod utils;

use crate::render::Render;
use gl;
use glutin::{
    ContextBuilder, Event, EventsLoop, GlContext, GlWindow, WindowBuilder,
    WindowEvent,
};
use std::error::Error;

const GAME_TITLE: &'static str = "Neo Pac-Man";
// 16.6ms per frame for 60 frames per second.
const FPS: i32 = 60;

fn main() -> Result<(), Box<Error>> {
    let mut event_loop = EventsLoop::new();

    let window = WindowBuilder::new().with_title(GAME_TITLE);
    let context = ContextBuilder::new();
    let gl_window = GlWindow::new(window, context, &event_loop)?;

    unsafe { gl_window.make_current()? }

    let render = Render::new(gl_window);

    let mut running = true;
    while running {
        let now = utils::now();

        // Process inputs.
        event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => running = false,
            _ => (),
        });

        // Render frame.
        render.draw();

        //
        // Synchronize loop
        //
        let sleep_time = utils::compute_sleep_duration(now, FPS)?;
        // If sleep_time is negative, we don't want to sleep the main
        // thread because the current tick already takes longer than 16ms.
        if sleep_time.is_some() {
            let time = sleep_time.unwrap();
            println!("Thread sleep for: {:?}", time);
            std::thread::sleep(time);
        } else {
            println!("Frame drop occurs here...");
        }
    }

    Ok(())
}
