use cgmath::Matrix4;

pub mod diffuse;

pub trait Shader: Sync + Send {
    fn prepare(&self);
    fn bind_uniforms(&self, model: &Matrix4<f32>,
                     view: &Matrix4<f32>,
                     projection: &Matrix4<f32>);
}