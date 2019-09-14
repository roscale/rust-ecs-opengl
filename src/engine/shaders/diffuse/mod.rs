use crate::gl_wrapper::texture_2d::*;
use super::*;
use crate::gl_wrapper::shader_compilation::*;
use std::ffi::CString;
use nalgebra_glm::{Vec3, Mat4};
use std::sync::Arc;
use crate::containers::CONTAINER;
use specs::ReadStorage;
use specs::join::Join;
use crate::ToVec3;

#[derive(Clone)]
pub(crate) enum PixelData {
    Color(Vec3),
    Texture(Arc<Texture2D>),
}

#[derive(Clone)]
pub struct DiffuseData {
    pub(crate) diffuse: PixelData,
    pub(crate) specular: PixelData,
    pub normal: Option<Arc<Texture2D>>,
    pub shininess: f32,
}

impl Default for DiffuseData {
    fn default() -> Self {
        DiffuseData {
            diffuse: PixelData::Color(0.7.to_vec3()),
            specular: PixelData::Color(0.5.to_vec3()),
            normal: None,
            shininess: 32.0,
        }
    }
}

impl ShaderData for DiffuseData {
    fn bind_model(&self, model: &Mat4) {
        let shader = CONTAINER.get_local::<DiffuseShader>();
        shader.bind_model(model);

        // Bind diffuse
        match &self.diffuse {
            PixelData::Texture(texture) => {
                texture.activate(0);
                shader.program.set_uniform1i("material.diffuse_texture", 0);
                shader.program.set_uniform1i("material.using_diffuse_texture", 1);

            },
            PixelData::Color(color) => {
                shader.program.set_uniform3f("material.diffuse_color", color.as_slice());
                shader.program.set_uniform1i("material.using_diffuse_texture", 0);
            }
        }

        // Bind specular
        match &self.specular {
            PixelData::Texture(texture) => {
                texture.activate(1);
                shader.program.set_uniform1i("material.specular_texture", 1);
                shader.program.set_uniform1i("material.using_specular_texture", 1);
            },
            PixelData::Color(color) => {
                shader.program.set_uniform3f("material.specular_color", color.as_slice());
                shader.program.set_uniform1i("material.using_specular_texture", 0);
            }
        }

        // Bind normal
        match &self.normal {
            Some(texture) => {
                texture.activate(2);
                shader.program.set_uniform1i("material.normal_texture", 2);
            },
            None => {
//                panic!("Normal texture doesn't exist");
//                shader.program.set_uniform1i("material.normal_texture", -1);
            }
        }

        // Bind shininess
        shader.program.set_uniform1f("material.shininess", self.shininess);
    }

    fn bind_lights(&self,
                   transforms: &ReadStorage<Transform>,
                   point_lights: &ReadStorage<PointLight>
    ) {
        let shader = CONTAINER.get_local::<DiffuseShader>();
        shader.bind_lights(transforms, point_lights);
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
}

impl Default for DiffuseShader {
    fn default() -> Self {
        DiffuseShader {
            program: Self::compile_program(),
        }
    }
}

impl DiffuseShader {
    fn bind_model(&self, model: &Mat4) {
        self.program.use_program();
        self.program.set_uniform_matrix4fv("model", model.as_ptr());
    }

    fn bind_lights(&self, transforms: &ReadStorage<Transform>, point_lights: &ReadStorage<PointLight>) {
        // TODO bind multiple lights
        for (transform, point_light) in (transforms, point_lights).join() {
            let transform = transform as &Transform;
            let point_light = point_light as &PointLight;

            self.program.set_uniform3f("light.position", transform.position.as_slice());
            self.program.set_uniform3f("light.color", point_light.color.as_slice());
            self.program.set_uniform1f("light.ambient_strength", 0.5);
            self.program.set_uniform1f("light.intensity", point_light.intensity);
            break;
        }
    }
}