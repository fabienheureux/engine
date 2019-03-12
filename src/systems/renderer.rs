use crate::GameState;
use crate::{
    components::{Light, Mesh, Transform},
    ecs::{Entity, System, World},
    opengl::OpenGL,
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

        let model = glm::Mat4::identity();
        let mut model = glm::translate(&model, &transform.position);

        model = glm::scale(&model, &transform.scale);

        let vao = mesh.get_vao();
        let shader = mesh.get_shader();
        let texture = mesh.get_texture();

        OpenGL::use_shader(shader.id);

        shader.set_matrix4("model", glm::value_ptr(&model));

        if let Some(light) = light {
            light.set_to_shader(state.lights_ubo, &transform);
        }

        shader.set_int("material.diffuse", texture.id as i32);
        shader.set_vec3("material.specular", &(0.5, 0.5, 0.5));
        shader.set_float("material.shininess", 32.);

        if mesh.has_ebo {
            OpenGL::draw_with_ebo(vao, Some(texture.id), 6);
        } else {
            OpenGL::draw(vao, Some(texture.id), 36);
        }
    }
}
