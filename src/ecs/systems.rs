use specs::prelude::*;
use specs::{System, WriteStorage, ReadStorage};
use crate::ecs::components::{Transform, Velocity, Shader};
use cgmath::{Matrix4, Point3, Vector3, Rad, Decomposed, Matrix, vec3, Deg};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut tr, vel): Self::SystemData) {
        for (tr, vel) in (&mut tr, &vel).join() {
            tr.position += vel.0;
        }
    }
}

pub struct MeshRenderer;

impl<'a> System<'a> for MeshRenderer {
    type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Shader>);

    fn run(&mut self, (transform, shader): Self::SystemData) {
        for (transform, shader) in (&transform, &shader).join() {
            let model_matrix = {
                let translate_matrix = Matrix4::from_translation(transform.position);

                let rotate_matrix = {
                    let rotate_matrix_x = Matrix4::from_angle_x(Rad(transform.rotation.x));
                    let rotate_matrix_y = Matrix4::from_angle_y(Rad(transform.rotation.y));
                    let rotate_matrix_z = Matrix4::from_angle_z(Rad(transform.rotation.z));
                    rotate_matrix_x * rotate_matrix_y * rotate_matrix_z
                };

                let scale_matrix: Matrix4<f32> = Matrix4::from_nonuniform_scale(transform.scale.x, transform.scale.y, transform.scale.y);
                translate_matrix * rotate_matrix * scale_matrix
            };

            let view_matrix = Matrix4::from_translation(vec3(0.0f32, 0.0, -3.0));
            let projection_matrix = cgmath::perspective(Deg(45.0f32), 800.0 / 800.0, 0.1, 100.0);

            let prog = &shader.0;

            prog.use_program();
            prog.set_uniform1i("myTexture", 0);
            prog.set_uniform_matrix4fv("model", Matrix4::as_ptr(&model_matrix));
            prog.set_uniform_matrix4fv("view", Matrix4::as_ptr(&view_matrix));
            prog.set_uniform_matrix4fv("projection", Matrix4::as_ptr(&projection_matrix));
        }
    }
}