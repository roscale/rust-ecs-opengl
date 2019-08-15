use nalgebra_glm::Vec3;
use super::*;
use crate::containers::global_instances::CONTAINER;
use crate::gl_wrapper::shader_compilation::{ShaderProgram, ShaderPart};
use std::ffi::CString;

#[derive(Clone)]
pub struct OutlineData {
    pub color: Vec3
}

impl ShaderData for OutlineData {
    fn bind_mvp(&self, model: &Mat4, view: &Mat4, projection: &Mat4, camera_pos: &Vec3) {
        let shader = CONTAINER.get_local::<OutlineShader>();
        shader.bind_mvp(model, view, projection, camera_pos);
        shader.program.set_uniform3f("color", self.color.as_slice());
    }

    fn bind_lights(&self, transforms: &ReadStorage<Transform>, point_lights: &ReadStorage<PointLight>) {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct OutlineShader {
    program: ShaderProgram,
}

impl OutlineShader {
    fn compile_program() -> ShaderProgram {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("outline.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("outline.frag")).unwrap()
        ).unwrap();

        ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap()
    }
}

impl Default for OutlineShader {
    fn default() -> Self {
        OutlineShader {
            program: Self::compile_program(),
        }
    }
}

impl OutlineShader {
    fn bind_mvp(&self, model: &Mat4,
                view: &Mat4,
                projection: &Mat4,
                camera_pos: &Vec3) {
        self.program.use_program();

        self.program.set_uniform_matrix4fv("model", model.as_ptr());
        self.program.set_uniform_matrix4fv("view", view.as_ptr());
        self.program.set_uniform_matrix4fv("projection", projection.as_ptr());
    }
}