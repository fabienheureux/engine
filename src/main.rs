mod camera;
mod constants;
mod game_loop;
mod helpers;
mod opengl;
mod primitives;
mod shader;
mod texture;
mod time;
mod watcher;
mod window;
mod world;

use crate::{
    camera::Camera, constants::RESSOURCE_PATH, game_loop::GameLoop,
    watcher::FileWatcher, window::Window, world::World,
};

fn main() -> Result<(), notify::Error> {
    let watcher = FileWatcher::new();
    let mut window = Window::new();
    let mut game_loop = GameLoop::new();
    let mut camera = Camera::new();

    let mut world = World::new();
    world.entities = helpers::load_entities();

    game_loop.start(|time| {
        window.capture();

        let running = !window.should_close;

        watcher.update(&mut world);

        camera.update(&window, &time);

        // Render frame.
        world.draw(&window, &mut camera);
        running
    });

    Ok(())
}
