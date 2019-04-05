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
mod opengl;
mod scene_loader;
mod shader;
mod systems;
mod time;
mod window;

use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    ecs::World,
    editor::Editor,
    game_loop::GameLoop,
    game_state::GameState,
    opengl::OpenGL,
    scene_loader::SceneLoader,
    shader::Shader,
    systems::{EditorCamera, Physic, Player, Renderer},
};

fn main() -> Result<(), notify::Error> {
    let mut state = GameState::new();
    let mut game_loop = GameLoop::new();
    let mut world = World::new();
    let mut editor = Editor::default();

    let mut scene_loader = SceneLoader::new(2);
    scene_loader.set_scene("scene_1.ron");

    // Load scene for the first time.
    scene_loader.load(&mut world, &mut state);

    // Add systems
    world.add_system(EditorCamera::default());
    world.add_system(Player::default());
    world.add_system(Physic::default());
    world.add_system(Renderer::default());

    game_loop.start(|time, fps| {
        state.window.capture();
        state.time = time.clone();

        editor.check_inputs(&mut state);

        scene_loader.watch(&mut world, &mut state);

        if editor.enabled_physics {
            state.physic_world.step();
        }

        let running = !state.window.should_close;

        // First render pass.
        // We render to the scene fbo.
        let (fbo, tex) = state.scene_fbo;
        OpenGL::use_fbo(fbo);
        OpenGL::set_depth_buffer(true);
        OpenGL::clear_color((0., 0., 0.));

        if editor.enabled_wireframe_mode {
            OpenGL::line_mode();
        }
        world.run(&mut state);
        OpenGL::fill_mode();

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
