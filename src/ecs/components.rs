use specs::prelude::*;
use cgmath::Vector3;
use crate::gl_wrapper::shader_compilation::Program;

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

pub struct Shader(pub Program);

impl Component for Shader {
    type Storage = VecStorage<Self>;
}

pub struct Velocity(pub Vector3<f32>);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}