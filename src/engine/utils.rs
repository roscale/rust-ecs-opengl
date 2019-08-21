use nalgebra_glm::{Vec3, vec3};

pub trait ToVec3 {
    fn to_vec3(&self) -> Vec3;
}

impl ToVec3 for f32 {
    fn to_vec3(&self) -> Vec3 {
        vec3(*self, *self, *self)
    }
}

impl ToVec3 for [f32; 3] {
    fn to_vec3(&self) -> Vec3 {
        vec3(self[0], self[1], self[2])
    }
}

impl ToVec3 for (f32, f32, f32) {
    fn to_vec3(&self) -> Vec3 {
        vec3(self.0, self.1, self.2)
    }
}