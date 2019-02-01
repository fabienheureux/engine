mod camera;
mod constants;
mod game_loop;
mod helpers;
mod opengl;
mod primitives;
mod shader;
mod texture;
mod time;
mod window;
mod world;

use crate::{
    camera::Camera, constants::RESSOURCE_PATH, game_loop::GameLoop,
    window::Window, world::World,
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

    // Watch the ressources folder every 2 secs.
    let (sender, receiver) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(sender, Duration::from_secs(2))?;
    watcher.watch(RESSOURCE_PATH, RecursiveMode::NonRecursive)?;

    game_loop.start(|time| {
        window.capture();

        let running = !window.should_close;

        if receiver.try_recv().is_ok() {
            world.entities = helpers::load_entities();
        }

        camera.update(&window, &time);

        // Render frame.
        world.draw(&window, &mut camera);
        running
    });

    Ok(())
}
