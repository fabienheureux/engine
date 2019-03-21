use crate::GameState;
use crate::{
    components::{Light, Mesh, Transform, Outline},
    ecs::{Entity, System, World},
    opengl::OpenGL,
    shader::Shader,
};
use nalgebra_glm as glm;
use std::any::TypeId;

#[derive(Debug, Default)]
pub struct Selectable;

impl System for Selectable {
    fn get_targets(&self) -> Vec<TypeId> {
        vec![World::get_type::<Mesh>(), World::get_type::<Outline>(), World::get_type::<Transform>()]
    }

    fn process(&self, entity: &mut Entity, state: &mut GameState) {
        let transform = entity.get::<Transform>();
        let mesh = entity.get::<Mesh>();
        let vao = mesh.get_vao();

        let model = glm::Mat4::identity();
        let mut model = glm::translate(&model, &transform.position);

        model = glm::scale(&model, &transform.scale);


        // Second render pass for outline.
        unsafe {
            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);
        }
        let outline_shader = state.asset_manager.get_one::<Shader>("outline");
        OpenGL::use_shader(outline_shader.id);

        let model = glm::scale(&model, &glm::vec3(1.1, 1.1, 1.1));
        outline_shader.set_matrix4("model", glm::value_ptr(&model));

        if mesh.has_ebo {
            OpenGL::draw_with_ebo(vao, None, mesh.lines);
        } else {
            OpenGL::draw(vao, None, mesh.lines);
        }

        unsafe {
            gl::StencilMask(0xFF);
            gl::Enable(gl::DEPTH_TEST);
        }
    }
}

