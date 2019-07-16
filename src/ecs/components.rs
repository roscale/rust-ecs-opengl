use specs::prelude::*;
use cgmath::Vector3;
use crate::shaders::Shader;

// TODO implement Default trait to all the components

#[derive(Component, Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

#[derive(Component, Debug)]
pub struct Velocity(pub Vector3<f32>);

#[derive(Component)]
pub struct Material {
    pub shader: Box<dyn Shader>,
}

#[derive(Component, Debug)]
pub struct Mesh(pub Vec<f32>);

#[derive(Component, Debug)]
pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32
}

#[derive(Component, Debug)]
pub struct DirLight {
    pub color: Vector3<f32>,
    pub range: f32,
    pub intensity: f32,
    pub direction: Vector3<f32>,
}

#[derive(Component, Debug)]
pub struct PointLight {
    pub color: Vector3<f32>,
    pub range: f32,
    pub intensity: f32
}

#[derive(Component, Debug)]
pub struct Spotlight {
    pub color: Vector3<f32>,
    pub range: f32,
    pub intensity: f32
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Input;