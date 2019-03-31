use crate::{
    fonts::GameFont,
    asset_manager::{AssetManager, Texture},
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    opengl::OpenGL,
    shader::Shader,
    time::Time,
    window::Window,
};
use nalgebra_glm as glm;

pub struct GameState {
    pub window: Window,
    pub time: Time,
    pub camera_ubo: u32,
    pub lights_ubo: u32,
    pub editor_mode: bool,
    pub asset_manager: AssetManager,

    pub screen_vao: u32,
    pub scene_fbo: (u32, u32),
    pub skybox: (u32, i32, u32),
    pub debug_text: GameFont,
    pub cam_pos: String,
}

impl GameState {
    pub fn new() -> Self {
        let window = Window::new();
        let projection = glm::perspective(
            45_f32.to_radians(),
            (SCREEN_WIDTH / SCREEN_HEIGHT) as f32,
            0.1,
            100.,
        );

        let camera_ubo = OpenGL::create_camera_ubo(0);
        OpenGL::set_mat4_to_ubo(projection, camera_ubo, 0);

        let lights_ubo = OpenGL::create_lights_ubo(1);

        let mut asset_manager = AssetManager::default();

        asset_manager.add_shader("default", "default", "default");
        asset_manager.add_shader(
            "default_material",
            "default_material",
            "default_material",
        );
        asset_manager.add_shader("light", "default", "default");
        asset_manager.add_shader("outline", "default_material", "outline");
        // TODO: Should rename those shaders.
        asset_manager.add_shader("screen_output", "quad", "quad");
        asset_manager.add_shader("skybox", "skybox", "skybox");
        // For text purpose.
        asset_manager.add_shader("text", "text", "text");

        // Load skybox data.
        // TODO: Load real skybox textures after fixing why it takes 
        // so much time to load.
        asset_manager.add_texture("skybox_up.png");

        let skybox: Vec<&Texture> = vec![
            asset_manager.get_ressource::<Texture>("skybox_up.png"),
            asset_manager.get_ressource::<Texture>("skybox_up.png"),
            asset_manager.get_ressource::<Texture>("skybox_up.png"),
            asset_manager.get_ressource::<Texture>("skybox_up.png"),
            asset_manager.get_ressource::<Texture>("skybox_up.png"),
            asset_manager.get_ressource::<Texture>("skybox_up.png"),
        ];

        let shaders = asset_manager.get_ressources::<Shader>();

        shaders.iter().for_each(|shader| {
            OpenGL::set_uniform_block(shader.id, 0, "Camera");
            OpenGL::set_uniform_block(shader.id, 1, "Lights");
        });

        let screen_vao = OpenGL::gen_screen_quad();
        let scene_fbo = OpenGL::create_fbo();

        let skybox = {
            let tex = OpenGL::load_cubemap(skybox);
            let data = OpenGL::gen_skybox();

            (data.0, data.1, tex)
        };

        let debug_text = GameFont::new(28.);

        Self {
            window,
            time: Time::default(),
            editor_mode: true,
            camera_ubo,
            lights_ubo,
            asset_manager,
            screen_vao,
            scene_fbo,
            skybox,
            debug_text,
            cam_pos: String::default(),
        }
    }
}
