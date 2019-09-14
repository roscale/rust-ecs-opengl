use crate::gl_wrapper::shader_compilation::{ShaderProgram, ShaderPart};
use std::ffi::CString;
use crate::gl_wrapper::texture_2d::Texture2D;

#[derive(Clone)]
pub struct KernelShader {
    program: ShaderProgram,
}

impl KernelShader {
    fn compile_program() -> ShaderProgram {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("../simple.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("kernel.frag")).unwrap()
        ).unwrap();

        ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap()
    }
}

impl Default for KernelShader {
    fn default() -> Self {
        KernelShader {
            program: Self::compile_program(),
        }
    }
}

impl KernelShader {
    pub fn bind_screen_texture(&self, texture: &Texture2D) {
        self.program.use_program();

        texture.activate(0);
        self.program.set_uniform1i("screen_texture", 0);
    }

    pub fn bind_kernel(&self, kernel: &[f32]) {
        self.program.set_uniform1fv("kernel", kernel);
        self.program.set_uniform1i("kernel_size", f32::sqrt(kernel.len() as f32) as i32);
    }
}