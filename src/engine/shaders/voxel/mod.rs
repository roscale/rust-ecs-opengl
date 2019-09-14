use crate::gl_wrapper::shader_compilation::{ShaderProgram, ShaderPart};
use std::ffi::CString;

#[derive(Clone)]
pub struct VoxelShader {
    program: ShaderProgram,
}

impl VoxelShader {
    fn compile_program() -> ShaderProgram {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("voxel.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("voxel.frag")).unwrap()
        ).unwrap();

        ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap()
    }

    pub fn bind(&self) {
        self.program.use_program();
    }
}

impl Default for VoxelShader {
    fn default() -> Self {
        VoxelShader {
            program: Self::compile_program(),
        }
    }
}
