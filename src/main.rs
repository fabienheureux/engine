/// This is a game engine in development. This engine is
/// the fondation for a pseudo-3D game.
///
/// TODO
///
/// ECS:
///  - Data should be flat in memory.
///  - It should be possible to have multi mutable ref of components
///    from our systems.
///
/// Asset Manager:
///  - Re-use id for the asset storage.
///  - Could we do better than cloning the key?
///  - We should guess the image extension.
///
/// Rendering:
///  - We should have a strong separation between opengl stuff and the
///    our engine.
///  - Add FBO.
///
/// ...
/// ...
///
/// - Release the game.
///
mod asset_manager;
mod components;
mod constants;
mod ecs;
mod editor;
mod game_loop;
mod game_state;
mod helpers;
mod opengl;
mod shader;
mod systems;
mod time;
mod window;

use crate::{
    shader::Shader,
    constants::SCENE_PATH,
    ecs::World,
    editor::Editor,
    game_loop::GameLoop,
    game_state::GameState,
    opengl::OpenGL,
    systems::{EditorCamera, Renderer},
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() -> Result<(), notify::Error> {
    let mut state = GameState::new();
    let mut game_loop = GameLoop::new();
    let mut world = World::new();

    // Load scene for the first time.
    world.load_entities(helpers::load_scene("scene_1.ron", &mut state));

    // Add systems
    world.add_system(EditorCamera::default());
    world.add_system(Renderer::default());

    // Watch the ressources folder every 2 secs.
    let (sender, receiver) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(sender, Duration::from_secs(2))?;
    watcher.watch(SCENE_PATH, RecursiveMode::NonRecursive)?;

    game_loop.start(|time| {
        Editor::run(&mut state);
        // First render pass. 
        // We render to the scene fbo.
        let (fbo, tex) = state.scene_fbo;
        OpenGL::use_fbo(fbo);
        OpenGL::set_depth_buffer(true);
        OpenGL::clear_color((0., 0., 0.));

        state.window.capture();
        state.time = time.clone();

        let running = !state.window.should_close;

        if receiver.try_recv().is_ok() {
            world.load_entities(helpers::load_scene("scene_1.ron", &mut state));
        }

        world.run(&mut state);

        // Final render pass with no post processing.
        // with the default framebuffer.
        OpenGL::use_fbo(0);
        OpenGL::clear_color((1., 1., 1.));

        let (_, shader) = state.asset_manager.get::<Shader>("screen_output");
        OpenGL::set_depth_buffer(false);
        OpenGL::use_shader(shader.id);
        shader.set_int("screen", 0);

        OpenGL::draw(state.screen_vao, Some(tex), 6);

        state.window.swap_gl();
        running
    });

    Ok(())
}
