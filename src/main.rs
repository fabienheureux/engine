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
mod fonts;
mod game_loop;
mod game_state;
mod helpers;
mod opengl;
mod shader;
mod systems;
mod time;
mod window;

use crate::{
    constants::{SCENE_PATH, SCREEN_HEIGHT, SCREEN_WIDTH},
    ecs::World,
    editor::Editor,
    game_loop::GameLoop,
    game_state::GameState,
    opengl::OpenGL,
    shader::Shader,
    systems::{EditorCamera, Renderer, Physic},
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::components::Collider;
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nalgebra_glm as glm;

fn main() -> Result<(), notify::Error> {
    let mut state = GameState::new();
    let mut game_loop = GameLoop::new();
    let mut world = World::new();
    let mut editor = Editor::default();


    // Ground test.
    let shape = ShapeHandle::new(Cuboid::new(glm::vec3(5., 0.04, 5.)));
    let position = glm::vec3(0., -0.5, 0.);
    Collider::new(&mut state.physic_world, shape, position, 1.);

    // Load scene for the first time.
    world.load_entities(helpers::load_scene("scene_1.ron", &mut state));

    // Add systems
    world.add_system(EditorCamera::default());
    world.add_system(Physic::default());
    world.add_system(Renderer::default());

    // Watch the ressources folder every 2 secs.
    let (sender, receiver) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(sender, Duration::from_secs(2))?;
    watcher.watch(SCENE_PATH, RecursiveMode::NonRecursive)?;

    game_loop.start(|time, fps| {
        state.window.capture();
        state.time = time.clone();

        editor.run(&mut state);

        if editor.enabled_physics {
            state.physic_world.step();
        }
        
        let running = !state.window.should_close;

        if receiver.try_recv().is_ok() {
            world.load_entities(helpers::load_scene("scene_1.ron", &mut state));
        }


        // First render pass.
        // We render to the scene fbo.
        let (fbo, tex) = state.scene_fbo;
        OpenGL::use_fbo(fbo);
        OpenGL::set_depth_buffer(true);
        OpenGL::clear_color((0., 0., 0.));

        world.run(&mut state);

        // Skybox pass.
        unsafe { gl::DepthFunc(gl::LEQUAL) }
        let shader = state.asset_manager.get_ressource::<Shader>("skybox");
        OpenGL::use_shader(shader.id);
        shader.set_int("skybox", 0);
        OpenGL::draw_skybox(state.skybox.0, state.skybox.2, state.skybox.1);
        unsafe { gl::DepthFunc(gl::LESS) }

        // For post effects, we're using the default framebuffer.
        OpenGL::use_fbo(0);
        OpenGL::clear_color((1., 1., 1.));

        let shader =
            state.asset_manager.get_ressource::<Shader>("screen_output");
        OpenGL::set_depth_buffer(false);
        OpenGL::use_shader(shader.id);
        shader.set_int("screen", 0);
        OpenGL::draw(state.screen_vao, Some(tex), 6);

        // HUD render pass.
        // For now, it is the last render pass 'cause it is easy to debug with
        // lines mode.
        let text_shader = state.asset_manager.get_ressource::<Shader>("text");
        state.debug_text.render(
            format!("fps: {}", fps.round()).as_str(),
            text_shader,
            (SCREEN_WIDTH - 105., SCREEN_HEIGHT - 60.),
            (255., 0., 0.),
        );
        state.debug_text.render(
            state.cam_pos.as_str(),
            text_shader,
            (SCREEN_WIDTH - 170., 0.),
            (255., 0., 0.),
        );

        state.window.swap_gl();
        running
    });

    Ok(())
}
