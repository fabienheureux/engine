use crate::GameState;
use crate::{
    components::{Light, Mesh, Transform},
    ecs::{Entity, System, World},
    opengl::OpenGL,
    shader::Shader,
};
use nalgebra_glm as glm;
use std::any::TypeId;

#[derive(Debug, Default)]
pub struct Renderer;

impl System for Renderer {
    fn get_targets(&self) -> Vec<TypeId> {
        vec![World::get_type::<Mesh>(), World::get_type::<Transform>()]
    }

    fn process(&self, entity: &mut Entity, state: &mut GameState) {
        let transform = entity.get::<Transform>();
        let mesh = entity.get::<Mesh>();
        let light = entity.get_opt::<Light>();

        let mut model = glm::Mat4::identity();

        model = glm::translate(&model, &transform.position);

        if let Some((axis, angle)) = transform.rotation.axis_angle() {
            model = glm::rotate(&model, angle, &axis);
        }

        model = glm::scale(&model, &transform.scale);

        let vao = mesh.get_vao();
        let shader = state.asset_manager.get_ressource::<Shader>(mesh.shader);
        let texture_key = mesh.get_texture();

        OpenGL::use_shader(shader.id);

        shader.set_matrix4("model", glm::value_ptr(&model));

        if let Some(light) = light {
            light.set_to_shader(state.lights_ubo, &transform);
        }

        if texture_key.is_some() {
            shader.set_int("material.diffuse", 0);
        }
        shader.set_vec3("material.specular", &(0.5, 0.5, 0.5));
        shader.set_float("material.shininess", 32.);
        shader.set_vec3("color", &mesh.color);

        let mut texture = None;
        if let Some(texture_key) = texture_key {
            texture = state.asset_manager.get_asset(texture_key.as_str()).gl_id;
        }

        if mesh.has_ebo {
            OpenGL::draw_with_ebo(vao, texture, mesh.lines);
        } else {
            OpenGL::draw(vao, texture, mesh.lines);
        }
    }
}
