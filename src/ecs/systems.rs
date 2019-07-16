use specs::prelude::*;
use specs::{System, WriteStorage, ReadStorage};
use crate::ecs::components::*;
use crate::ecs::resources::*;
use nalgebra_glm::{Vec2, Vec3, vec2, Mat4, vec3};
use std::ops::Deref;

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

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (Write<'a, InputEventQueue>,
                       ReadStorage<'a, Input>,
                       WriteStorage<'a, Transform>,
                       ReadStorage<'a, Camera>,
                       Read<'a, ActiveCamera>,
                       Write<'a, InputCache>);

    fn run(&mut self, (mut input_event_queue, inputs, mut transforms, cameras, active_camera, mut input_cache): Self::SystemData) {
        let active_camera = active_camera.entity.unwrap();
        let camera = cameras.get(active_camera).unwrap();
        let transform = transforms.get_mut(active_camera).unwrap();

        while let Some(ref event) = input_event_queue.queue.pop_front() {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = *x as f32;
                    let y = *y as f32;
                    let current_pos = vec2(x, y);
                    input_cache.cursor_rel_pos = current_pos - input_cache.last_cursor_pos;
                    input_cache.last_cursor_pos = vec2(x, y);
                    let (x, y) = (input_cache.cursor_rel_pos.x, input_cache.cursor_rel_pos.y);
                    println!("x: {} y: {}", x, y);

                    transform.rotation.y += x * 0.001;
                    transform.rotation.x -= y * 0.001;
                }

//                glfw::WindowEvent::CursorEnter(enter) => {
//                    if *enter {
//                        input_cache.cursor_rel_pos = vec2(0.0, 0.0);
//                        input_cache.last_cursor_pos = vec2(0.0, 0.0);
//                    }
//                }
                _ => {}
            }
        }
    }
}

pub struct MeshRenderer;

use crate::shaders::Shader;
use specs::storage::UnprotectedStorage;
use nalgebra::{Vector3, Translation, Matrix4, Translation3, Point3};

impl<'a> System<'a> for MeshRenderer {
    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, Material>,
                       ReadStorage<'a, Mesh>,
                       ReadStorage<'a, Material>,
                       ReadStorage<'a, Camera>,
                       Read<'a, ActiveCamera>,
                       ReadStorage<'a, PointLight>);

    fn run(&mut self, (transform, shader, mesh, material, camera, active_camera, point_lights): Self::SystemData) {
        let (camera, cam_tr) = match active_camera.entity {
            Some(e) => (
                camera.get(e).expect("Active camera must have a Camera component"),
                transform.get(e).expect("Active camera must have a Transform component")
            ),
            None => return
        };

        let mut direction = vec3(
            cam_tr.rotation.x.cos() * cam_tr.rotation.y.cos(),
            cam_tr.rotation.x.sin(),
            cam_tr.rotation.x.cos() * cam_tr.rotation.y.sin(),
        );

        let look_at = nalgebra_glm::look_at(&cam_tr.position, &(cam_tr.position + direction), &Vector3::y());
        let view_matrix = look_at.prepend_translation(&(-cam_tr.position));

        let projection_matrix = nalgebra_glm::perspective(1f32, camera.fov, camera.near_plane, camera.far_plane);

        for light in (&point_lights).join() {
            println!("Range: {}", light.range);
        }

        for (transform, shader, mesh, material) in (&transform, &shader, &mesh, &material).join() {
            let model_matrix = {
                let translate_matrix = Matrix4::new_translation(&transform.position);

                let rotate_matrix = Matrix4::from_euler_angles(
                    transform.rotation.x,
                    transform.rotation.y,
                    transform.rotation.z,
                );

                let scale_matrix: Mat4 = Matrix4::new_nonuniform_scaling(&transform.scale);
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