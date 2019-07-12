use crate::gl_wrapper::texture_2d::*;
use super::Shader;
use crate::gl_wrapper::shader_compilation::*;
use std::ffi::CString;
use cgmath::{Matrix4, Matrix, Vector3, Vector4};

pub struct DiffuseShader {
    program: ShaderProgram,
    pub diffuse_texture: Texture2D,
    pub light_color: Vector3<f32>,
    pub ambient_strength: f32,
    pub specular_strength: f32,
    pub shininess: f32,

}

impl DiffuseShader {
    pub fn new(diffuse_texture: Texture2D,
               light_color: Vector3<f32>,
               ambient_strength: f32,
               specular_strength: f32,
               shininess: f32) -> Self {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("diffuse.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("diffuse.frag")).unwrap()
        ).unwrap();

        let program = ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap();

        DiffuseShader {
            program,
            diffuse_texture,
            light_color,
            ambient_strength,
            specular_strength,
            shininess
        }
    }
}

impl Shader for DiffuseShader {
    fn prepare(&self) {
        Texture2D::activate(0);
        self.diffuse_texture.bind();
        self.program.use_program();
    }

    fn bind_uniforms(&self, model: &Matrix4<f32>,
                     view: &Matrix4<f32>,
                     projection: &Matrix4<f32>) {
        self.program.set_uniform_matrix4fv("model", Matrix4::as_ptr(model));
        self.program.set_uniform_matrix4fv("view", Matrix4::as_ptr(view));
        self.program.set_uniform_matrix4fv("projection", Matrix4::as_ptr(projection));
        self.program.set_uniform1i("diffuse", 0);

        let light_view_space: Vector4<f32> = view * Vector4 {
            x: 0.0f32,
            y: 0.0,
            z: 5.0,
            w: 1.0,
        };
        self.program.set_uniform3f("light_pos", &[
            light_view_space.x, light_view_space.y, light_view_space.z,
        ]);
        self.program.set_uniform3f("light_color", &[
            self.light_color.x, self.light_color.y, self.light_color.z
        ]);
        self.program.set_uniform1f("ambient_strength", self.ambient_strength);
        self.program.set_uniform1f("specular_strength", self.specular_strength);
        self.program.set_uniform1f("shininess", self.shininess);
    }
}