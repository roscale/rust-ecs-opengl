use nalgebra_glm::{Mat4, Vec3};
use specs::ReadStorage;
use crate::ecs::components::*;

pub mod diffuse;
pub mod outline;
pub mod post_processing;
pub mod cube_map;
pub mod voxel;

pub trait ShaderData: Sync + Send {
    fn bind_model(&self, model: &Mat4);

    fn bind_lights(&self,
                   transforms: &ReadStorage<Transform>,
                   point_lights: &ReadStorage<PointLight>
    );
}