use specs::prelude::*;
use nalgebra_glm::{Vec3, vec3, Mat4, Mat3};
use crate::shaders::*;
use nalgebra::{Matrix4, Point3, Point, Vector, Vector3};
use crate::gl_wrapper::vao::VAO;
use std::sync::Arc;
use nphysics3d::material::BasicMaterial;
use nphysics3d::object::{BodyStatus, ActivationStatus, RigidBodyDesc};
use nphysics3d::algebra::Velocity3;
use ncollide3d::shape::ShapeHandle;
use crate::post_processing_effects::PPEffect;
use crate::gl_wrapper::fbo::FBO;
use crate::gl_wrapper::texture_2d::Texture2D;
use crate::gl_wrapper::rbo::RBO;

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

#[derive(Component, Clone)]
pub struct MeshRenderer {
    pub mesh: Arc<Mesh>,
    pub material: Arc<Material>
}

pub struct Material {
    pub shader_data: Box<dyn ShaderData>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vao: VAO,
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>
}

#[derive(Debug)]
pub struct BoxCollider {
    pub box_size: Vec3,
    pub material: BasicMaterial<f32>,
}

impl Component for BoxCollider {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Debug)]
pub struct RigidBody {
    pub name: String,
    pub gravity_enabled: bool,
    pub status: BodyStatus,
    pub velocity: Velocity3<f32>,
    pub angular_inertia: Mat3,
    pub mass: f32,
    pub local_center_of_mass: Point3<f32>,
    pub sleep_threshold: Option<f32>,
    pub kinematic_translations: Vector3<bool>,
    pub kinematic_rotations: Vector3<bool>,
}

impl Component for RigidBody {
    type Storage = FlaggedStorage<Self>;
}

impl Default for RigidBody {
    fn default() -> Self {
        RigidBody {
            name: "".to_owned(),
            gravity_enabled: true,
            status: BodyStatus::Dynamic,
            velocity: Velocity3::zero(),
            angular_inertia: Mat3::zeros(),
            mass: 0.0,
            local_center_of_mass: Point::origin(),
            sleep_threshold: Some(ActivationStatus::<f32>::default_threshold()),
            kinematic_translations: Vector3::repeat(false),
            kinematic_rotations: Vector3::repeat(false),
        }
    }
}

pub struct Collider {
    pub shape: ShapeHandle<f32>,
    pub material: BasicMaterial<f32>,
}

impl Component for Collider {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Component)]
pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,

    pub post_processing_effects: Vec<Box<dyn PPEffect>>,
    pub fb: FBO
}

impl Camera {
    pub fn new(fov: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32, post_processing_effects: Vec<Box<dyn PPEffect>>) -> Self {
        let color_texture = Texture2D::new();
        color_texture.bind();
        color_texture.allocate_color(800, 800);

        let depth_stencil_rb: RBO = RBO::new();
        depth_stencil_rb.bind();
        depth_stencil_rb.create_depth_stencil(800, 800);

        // TODO Prefer composition over setters
        let mut fb = FBO::new();
        fb.bind();
        fb.attach_color_texture(&color_texture);
        fb.attach_depth_stencil_renderbuffer(&depth_stencil_rb);

        Camera {
            fov,
            aspect_ratio,
            near_plane,
            far_plane,
            post_processing_effects,
            fb
        }
    }
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

#[derive(Component)]
pub struct Outliner {
    pub scale: f32,
    pub color: Vec3,
}