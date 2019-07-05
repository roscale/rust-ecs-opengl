use specs::prelude::*;
use cgmath::{Vector3};
use crate::gl_wrapper::shader_compilation::Program;

pub struct Position(pub Vector3<f32>);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Rotation(pub Vector3<f32>);
impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

pub struct Scale(pub Vector3<f32>);
impl Component for Scale {
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