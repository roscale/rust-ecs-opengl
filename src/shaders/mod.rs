use nalgebra_glm::Mat4;

pub mod diffuse;

pub trait Shader: Sync + Send {
    fn prepare(&self);
    fn bind_uniforms(&self, model: &Mat4,
                     view: &Mat4,
                     projection: &Mat4);
}