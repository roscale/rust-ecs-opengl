use crate::gl_wrapper::shader_compilation::{ShaderProgram, ShaderPart};
use std::ffi::CString;
use nalgebra_glm::Mat4;

#[derive(Clone)]
pub struct CubeMapShader {
    program: ShaderProgram,
}

impl CubeMapShader {
    fn compile_program() -> ShaderProgram {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("cube_map.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("cube_map.frag")).unwrap()
        ).unwrap();

        ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap()
    }

    pub fn bind(&self, view: &Mat4, projection: &Mat4) {
        self.program.use_program();
        self.program.set_uniform_matrix4fv("view", view.as_ptr());
        self.program.set_uniform_matrix4fv("projection", projection.as_ptr());
        self.program.set_uniform1i("cube_map", 0);
    }
}

impl Default for CubeMapShader {
    fn default() -> Self {
        CubeMapShader {
            program: Self::compile_program(),
        }
    }
}
