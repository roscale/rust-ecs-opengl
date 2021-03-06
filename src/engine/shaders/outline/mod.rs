use nalgebra_glm::Vec3;
use super::*;
use crate::containers::CONTAINER;
use crate::gl_wrapper::shader_compilation::{ShaderProgram, ShaderPart};
use std::ffi::CString;

#[derive(Clone)]
pub struct OutlineData {
    pub color: Vec3
}

impl ShaderData for OutlineData {
    fn bind_model(&self, model: &Mat4) {
        let shader = CONTAINER.get_local::<OutlineShader>();
        shader.bind_model(model);
        shader.program.set_uniform3f("color", self.color.as_slice());
    }

    fn bind_lights(&self, _transforms: &ReadStorage<Transform>, _point_lights: &ReadStorage<PointLight>) {
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
    fn bind_model(&self, model: &Mat4) {
        self.program.use_program();
        self.program.set_uniform_matrix4fv("model", model.as_ptr());
    }
}