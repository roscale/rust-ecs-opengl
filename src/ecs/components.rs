use specs::prelude::*;
use nalgebra_glm::{Vec3, vec3, Mat4};
use crate::shaders::Shader;
use nalgebra::Matrix4;
use crate::gl_wrapper::vao::VAO;

// TODO implement Default trait to all the components

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub model_matrix: Mat4
}

impl Component for Transform {
    type Storage = FlaggedStorage<Self>;
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
            model_matrix: Matrix4::identity()
        }
    }
}

impl Transform {
    pub fn forward(&self) -> Vec3 {
        vec3(
            self.rotation.x.cos() * self.rotation.y.cos(),
            self.rotation.x.sin(),
            self.rotation.x.cos() * self.rotation.y.sin(),
        )
    }
}

#[derive(Component, Debug)]
pub struct TransformCache {

}

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct MeshRenderer {
    pub mesh: Mesh,
    pub material: Material
}

pub struct Material {
    pub shader: Box<dyn Shader>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vao: VAO,
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>
}

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