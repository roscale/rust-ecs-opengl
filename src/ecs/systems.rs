use specs::prelude::*;
use specs::{System, WriteStorage, ReadStorage};
use crate::ecs::components::{Position, Rotation, Scale, Velocity, Shader};
use cgmath::{Matrix4, Point3, Vector3, Rad, Matrix};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }
}

pub struct MeshRenderer;

impl<'a> System<'a> for MeshRenderer {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Rotation>, ReadStorage<'a, Scale>, ReadStorage<'a, Shader>);

    fn run(&mut self, (pos, rot, sc, sh): Self::SystemData) {
        for (pos, rot, sc, sh) in (&pos, &rot, &sc, &sh).join() {
            let translate_matrix = Matrix4::from_translation(pos.0);

            let rotate_matrix = {
                let rotate_matrix_x = Matrix4::from_angle_x(Rad(rot.0.x));
                let rotate_matrix_y = Matrix4::from_angle_y(Rad(rot.0.y));
                let rotate_matrix_z = Matrix4::from_angle_z(Rad(rot.0.z));
                rotate_matrix_x * rotate_matrix_y * rotate_matrix_z
            };

            let scale_matrix: Matrix4<f32> = Matrix4::from_nonuniform_scale(sc.0.x, sc.0.y, sc.0.y);
            let model_matrix = translate_matrix * rotate_matrix * scale_matrix;

//            println!("{:?}", model_matrix);

            let prog = &sh.0;

            prog.use_program();
            prog.set_uniform1i("myTexture", 0);
            prog.set_uniform_matrix4fv("model_matrix", Matrix4::as_ptr(&model_matrix));
        }
    }
}