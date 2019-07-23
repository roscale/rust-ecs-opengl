use nalgebra_glm::Mat4;

pub mod diffuse;

pub trait Shader: Sync + Send {
    fn bind_uniforms(&self, model: &Mat4,
                     view: &Mat4,
                     projection: &Mat4);
}

pub trait ShaderData: Sync + Send {
    fn bind_shader_uniforms(&self, model: &Mat4,
                     view: &Mat4,
                     projection: &Mat4);
}