use nalgebra_glm::Vec3;
use super::*;
use crate::containers::global_instances::CONTAINER;
use crate::gl_wrapper::shader_compilation::{ShaderProgram, ShaderPart};
use std::ffi::CString;
use crate::gl_wrapper::texture_2d::Texture2D;

#[derive(Clone)]
pub struct PostProcessingShader {
    program: ShaderProgram,
}

impl PostProcessingShader {
    fn compile_program() -> ShaderProgram {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("post_processing.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("post_processing.frag")).unwrap()
        ).unwrap();

        ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap()
    }
}

impl Default for PostProcessingShader {
    fn default() -> Self {
        PostProcessingShader {
            program: Self::compile_program(),
        }
    }
}

impl PostProcessingShader {
    pub fn bind_screen_texture(&self, texture: &Texture2D) {
        self.program.use_program();

        Texture2D::activate(0);
        texture.bind();
        self.program.set_uniform1i("screen_texture", 0);
    }
}