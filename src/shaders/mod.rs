use nalgebra_glm::{Mat4, Vec3};
use specs::ReadStorage;
use crate::ecs::components::*;

pub mod diffuse;
pub mod outline;
pub mod post_processing;

pub trait ShaderData: Sync + Send {
    fn bind_mvp(&self,
                model: &Mat4,
                view: &Mat4,
                projection: &Mat4,
                camera_pos: &Vec3
    );

    fn bind_lights(&self,
                   transforms: &ReadStorage<Transform>,
                   point_lights: &ReadStorage<PointLight>
    );
}