use specs::prelude::*;
use specs::{System, WriteStorage, ReadStorage};
use crate::ecs::components::{Transform, Velocity, Material, Mesh, Camera};
use crate::ecs::resources::ActiveCamera;
use cgmath::{Matrix4, Point3, Vector3, Rad, Decomposed, Matrix, vec3, Deg, Angle, InnerSpace};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut tr, vel): Self::SystemData) {
        for (tr, vel) in (&mut tr, &vel).join() {
            tr.position += vel.0;
//            tr.rotation.y += 0.01f32;
//            println!("{}", tr.rotation.y)
        }
    }
}

pub struct MeshRenderer;

use crate::shaders::Shader;
use specs::storage::UnprotectedStorage;

impl<'a> System<'a> for MeshRenderer {
    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, Material>,
                       ReadStorage<'a, Mesh>,
                       ReadStorage<'a, Material>,
                       ReadStorage<'a, Camera>,
                       Read<'a, ActiveCamera>);

    fn run(&mut self, (transform, shader, mesh, material, camera, active_camera): Self::SystemData) {
        let (camera, cam_tr) = match active_camera.entity {
            Some(e) => (
                camera.get(e).expect("Active camera must have a Camera component"),
                transform.get(e).expect("Active camera must have a Transform component")
            ),
            None => return
        };

        let direction = Vector3 {
            x: Rad(cam_tr.rotation.x).cos() * Rad(cam_tr.rotation.y).cos(),
            y: Rad(cam_tr.rotation.x).sin(),
            z: Rad(cam_tr.rotation.x).cos() * Rad(cam_tr.rotation.y).sin(),
        };

        let up = vec3(0.0f32, 1.0, 0.0);
//        let cam_right = Vector3::normalize(Vector3::cross(up, direction));
//        let cam_up = Vector3::cross(direction, cam_right);

        let look_at: Matrix4<f32> = Matrix4::look_at_dir(Point3 {
            x: cam_tr.position.x,
            y: cam_tr.position.y,
            z: cam_tr.position.z,
        }, direction, up);

        let view_matrix = look_at * Matrix4::from_translation(-cam_tr.position);
        let projection_matrix = cgmath::perspective(
            Deg(camera.fov),
            camera.aspect_ratio,
            camera.near_plane,
            camera.far_plane,
        );

        for (transform, shader, mesh, material) in (&transform, &shader, &mesh, &material).join() {
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

            let material = material as &Material;
            let ref shader = material.shader;
            shader.prepare();
            shader.bind_uniforms(&model_matrix, &view_matrix, &projection_matrix);
            gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, mesh.0.len() as i32));
        }
    }
}