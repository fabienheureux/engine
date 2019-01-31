mod camera;
mod constants;
mod game_loop;
mod helpers;
mod opengl;
mod shader;
mod texture;
mod time;
mod window;
mod world;

use crate::{
    camera::Camera, game_loop::GameLoop, window::Window, world::World,
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() -> Result<(), notify::Error> {
    let mut window = Window::new();
    let mut game_loop = GameLoop::new();
    let mut camera = Camera::new();

    let mut world = World::new();
    world.entities = helpers::load_entities();

    // Watch the ressources folder.
    let (sender, receiver) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(sender.clone(), Duration::from_secs(2))?;
    watcher.watch("assets/ressources/", RecursiveMode::NonRecursive)?;

    game_loop.start(|time| {
        window.capture();

        let running = !window.should_close;

        if let Ok(_) = receiver.try_recv() {
            world.entities = helpers::load_entities();
        }

        camera.update(&window, &time);

        // Render frame.
        world.draw(&window, &mut camera);
        running
    });

    Ok(())
}
