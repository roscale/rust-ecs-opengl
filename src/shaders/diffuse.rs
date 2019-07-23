use crate::gl_wrapper::texture_2d::*;
use super::*;
use crate::gl_wrapper::shader_compilation::*;
use std::ffi::CString;
use nalgebra_glm::{Vec3, Mat4, Vec4, vec4, vec3};
use std::sync::Arc;
use crate::containers::global_instances::CONTAINER;

#[derive(Clone)]
pub enum DiffuseData {
    Textures {
        // TODO Rc<Texture2D>
        diffuse_texture: Arc<Texture2D>,
        specular_texture: Arc<Texture2D>,
        normal_texture: Arc<Texture2D>,
        shininess: f32,
    },
    Colors {
        diffuse_color: Vec3,
        specular_color: Vec3,
        shininess: f32,
    },
}

impl Default for DiffuseData {
    fn default() -> Self {
        return DiffuseData::Colors {
            diffuse_color: vec3(0.7, 0.7, 0.7),
            specular_color: vec3(0.5, 0.5, 0.5),
            shininess: 32.0
        }
    }
}

impl ShaderData for DiffuseData {
    fn bind_shader_uniforms(&self, model: &Mat4, view: &Mat4, projection: &Mat4) {
        let shader = CONTAINER.get_local::<DiffuseShader>();
        shader.bind_uniforms(model, view, projection);

        match &self {
            DiffuseData::Textures {
                diffuse_texture,
                specular_texture,
                normal_texture,
                shininess,
            } => {
                Texture2D::activate(0);
                diffuse_texture.bind();

                Texture2D::activate(1);
                specular_texture.bind();

                Texture2D::activate(2);
                normal_texture.bind();

                shader.program.set_uniform1i("material.using_textures", 1);
                shader.program.set_uniform1i("material.diffuse_texture", 0);
                shader.program.set_uniform1i("material.specular_texture", 1);
                shader.program.set_uniform1i("material.normal_texture", 2);

                shader.program.set_uniform1f("material.shininess", *shininess);
            },
            DiffuseData::Colors {
                diffuse_color,
                specular_color,
                shininess
            } => {
                shader.program.set_uniform1i("material.using_textures", 0);
                shader.program.set_uniform3f("material.diffuse_color", diffuse_color.as_slice());
                shader.program.set_uniform3f("material.specular_color", specular_color.as_slice());

                shader.program.set_uniform1f("material.shininess", *shininess);
            },
        }
    }
}

#[derive(Clone)]
pub struct DiffuseShader {
    program: ShaderProgram,
}

impl DiffuseShader {
    fn compile_program() -> ShaderProgram {
        let vert_shader = ShaderPart::from_vert_source(
            &CString::new(include_str!("diffuse.vert")).unwrap()
        ).unwrap();

        let frag_shader = ShaderPart::from_frag_source(
            &CString::new(include_str!("diffuse.frag")).unwrap()
        ).unwrap();

        ShaderProgram::from_shaders(vert_shader, frag_shader).unwrap()
    }

//    pub fn new_with_textures(diffuse_texture: Arc<Texture2D>,
//                             specular_texture: Arc<Texture2D>,
//                             shininess: f32) -> Self {
//        DiffuseShader {
//            program: Self::compile_program(),
//            data: Some(Data::Textures {
//                diffuse_texture,
//                specular_texture,
//                shininess,
//            }),
//        }
//    }

//    pub fn new_without_textures(diffuse_color: Vec3,
//                                specular_color: Vec3,
//                                shininess: f32) -> Self {
//        DiffuseShader {
//            program: Self::compile_program(),
//            data: Some(Data::Colors {
//                diffuse_color,
//                specular_color,
//                shininess,
//            }),
//        }
//    }
}

impl Default for DiffuseShader {
    fn default() -> Self {
        DiffuseShader {
            program: Self::compile_program(),
        }
    }
}

impl Shader for DiffuseShader {
    fn bind_uniforms(&self, model: &Mat4,
                     view: &Mat4,
                     projection: &Mat4) {

        self.program.use_program();

        self.program.set_uniform_matrix4fv("model", model.as_ptr());
        self.program.set_uniform_matrix4fv("view", view.as_ptr());
        self.program.set_uniform_matrix4fv("projection", projection.as_ptr());

        let light_view_space: Vec4 = view * vec4(
            0.0f32,
            0.0,
            -5.0,
            1.0,
        );

        // TODO Modular lights
        self.program.set_uniform3f("light.position", &[
            light_view_space.x, light_view_space.y, light_view_space.z,
        ]);
        self.program.set_uniform3f("light.color", &[
            1.0, 1.0, 1.0
        ]);
        self.program.set_uniform1f("light.ambient_strength", 1.0);
        self.program.set_uniform1f("light.intensity", 1.0);
    }
}