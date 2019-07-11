use specs::prelude::*;
use cgmath::Vector3;
use crate::shaders::Shader;

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

pub struct Velocity(pub Vector3<f32>);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub struct Material {
    pub shader: Box<dyn Shader>,
}

impl Component for Material {
    type Storage = VecStorage<Self>;
}

pub struct Mesh(pub Vec<f32>);

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}

pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}
