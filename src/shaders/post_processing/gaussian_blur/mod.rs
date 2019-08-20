use crate::gl_wrapper::shader_compilation::{ShaderProgram, ShaderPart};
use std::ffi::CString;
use crate::gl_wrapper::texture_2d::Texture2D;

#[derive(Clone)]
pub struct GaussianBlurShader {
    program: ShaderProgram,
}

impl GaussianBlurShader {
    fn compile_program() -> ShaderProgram {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("../simple.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("blur.frag")).unwrap()
        ).unwrap();

        ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap()
    }
}

impl Default for GaussianBlurShader {
    fn default() -> Self {
        GaussianBlurShader {
            program: Self::compile_program(),
        }
    }
}

impl GaussianBlurShader {
    pub fn bind_screen_texture(&self, texture: &Texture2D) {
        self.program.use_program();

        Texture2D::activate(0);
        texture.bind();
        self.program.set_uniform1i("screen_texture", 0);
    }

    pub fn bind_kernel(&self, kernel: &[f32], vertical: bool) {
        self.program.set_uniform1fv("row", kernel);
        self.program.set_uniform1i("row_size", kernel.len() as i32);
        self.program.set_uniform1i("vertical", vertical as i32);
    }
}
