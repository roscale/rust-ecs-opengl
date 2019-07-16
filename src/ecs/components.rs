use specs::prelude::*;
use nalgebra_glm::Vec3;
use crate::shaders::Shader;

// TODO implement Default trait to all the components

#[derive(Component, Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

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
    pub color: Vec3,
    pub range: f32,
    pub intensity: f32,
    pub direction: Vec3,
}

#[derive(Component, Debug)]
pub struct PointLight {
    pub color: Vec3,
    pub range: f32,
    pub intensity: f32
}

#[derive(Component, Debug)]
pub struct Spotlight {
    pub color: Vec3,
    pub range: f32,
    pub intensity: f32
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Input;