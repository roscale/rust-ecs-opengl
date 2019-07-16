use crate::gl_wrapper::texture_2d::*;
use super::Shader;
use crate::gl_wrapper::shader_compilation::*;
use std::ffi::CString;
use nalgebra_glm::{Vec3, Mat4, Vec4, vec4};

pub struct DiffuseShader {
    program: ShaderProgram,
    pub diffuse_texture: Texture2D,
    pub specular_texture: Texture2D,
    pub light_color: Vec3,
    pub ambient_strength: f32,
    pub intensity: f32,
    pub shininess: f32,

}

impl DiffuseShader {
    pub fn new(diffuse_texture: Texture2D,
               specular_texture: Texture2D,
               light_color: Vec3,
               ambient_strength: f32,
               intensity: f32,
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
            specular_texture,
            light_color,
            ambient_strength,
            intensity,
            shininess
        }
    }
}

impl Shader for DiffuseShader {
    fn prepare(&self) {
        Texture2D::activate(0);
        self.diffuse_texture.bind();

        Texture2D::activate(1);
        self.specular_texture.bind();

        self.program.use_program();
    }

    fn bind_uniforms(&self, model: &Mat4,
                     view: &Mat4,
                     projection: &Mat4) {
        self.program.set_uniform_matrix4fv("model", model.as_ptr());
        self.program.set_uniform_matrix4fv("view", view.as_ptr());
        self.program.set_uniform_matrix4fv("projection", projection.as_ptr());
        self.program.set_uniform1i("material.diffuse", 0);
        self.program.set_uniform1i("material.specular", 1);
        self.program.set_uniform1f("material.shininess", self.shininess);

        let light_view_space: Vec4 = view * vec4(
            0.0f32,
            0.0,
            5.0,
            1.0,
        );
        self.program.set_uniform3f("light.position", &[
            light_view_space.x, light_view_space.y, light_view_space.z,
        ]);
        self.program.set_uniform3f("light.color", &[
            self.light_color.x, self.light_color.y, self.light_color.z
        ]);
        self.program.set_uniform1f("light.ambient_strength", self.ambient_strength);
        self.program.set_uniform1f("light.intensity", self.intensity);
    }
}