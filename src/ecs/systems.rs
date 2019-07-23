use specs::prelude::*;
use specs::{System, WriteStorage, ReadStorage};
use crate::ecs::components::*;
use crate::ecs::resources::*;
use nalgebra_glm::{vec2, Mat4, vec3};
use std::ffi::c_void;

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

pub struct TransformSystem {
    pub reader_id: ReaderId<ComponentEvent>,
    pub dirty: BitSet,
}

impl<'a> System<'a> for TransformSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Transform>);

    fn run(&mut self, (entities, mut transforms): Self::SystemData) {
        self.dirty.clear();
        let events = transforms.channel().read(&mut self.reader_id);

        for event in events {
//            println!("EVENT: {:#?}", *event);
            match event {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    self.dirty.add(*id);
                }
                ComponentEvent::Removed(_) => (),
            }
        }

        for (mut transforms, en) in (&mut transforms.restrict_mut(), &self.dirty).join() {
            let mut transform = transforms.get_mut_unchecked();

            transform.model_matrix = {
                let translate_matrix = Matrix4::new_translation(&transform.position);

                let rotate_matrix = Matrix4::from_euler_angles(
                    transform.rotation.x,
                    transform.rotation.y,
                    transform.rotation.z,
                );

                let scale_matrix: Mat4 = Matrix4::new_nonuniform_scaling(&transform.scale);
                translate_matrix * rotate_matrix * scale_matrix
            };
        }

        // Workaround for unflagging the components
        transforms.channel().read(&mut self.reader_id);
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

        while let Some(ref event) = input_event_queue.queue.pop_front() {
            let transform = transforms.get_mut(active_camera).unwrap();

            match event {
                WindowEvent::CursorPos(x, y) => {
                    let x = *x as f32;
                    let y = *y as f32;
                    let current_pos = vec2(x, y);
                    input_cache.cursor_rel_pos = current_pos - input_cache.last_cursor_pos;
                    input_cache.last_cursor_pos = vec2(x, y);
                    let (x, y) = (input_cache.cursor_rel_pos.x, input_cache.cursor_rel_pos.y);
//                    println!("x: {} y: {}", x, y);

                    transform.rotation.y += x * 0.001;
                    transform.rotation.x -= y * 0.001;
                }

                WindowEvent::Key(key, _, action, _) => {
                    input_cache.key_states.insert(*key, *action);
                }

                _ => {}
            }
        }

        if input_cache.is_key_pressed(Key::W) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position += transform.forward().scale(0.1f32);
        }

        if input_cache.is_key_pressed(Key::S) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position -= transform.forward().scale(0.1f32);
        }

        if input_cache.is_key_pressed(Key::A) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position -= transform.forward().cross(&Vector3::y()).scale(0.1f32);
        }

        if input_cache.is_key_pressed(Key::D) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position += transform.forward().cross(&Vector3::y()).scale(0.1f32);
        }
    }
}

pub struct MeshRendererSystem;

use nalgebra::{Vector3, Matrix4};
use glfw::{Key, WindowEvent};

impl<'a> System<'a> for MeshRendererSystem {
    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, MeshRenderer>,
                       ReadStorage<'a, Camera>,
                       Read<'a, ActiveCamera>,
                       ReadStorage<'a, PointLight>);

    fn run(&mut self, (transform, mesh_renderer, camera, active_camera, point_lights): Self::SystemData) {
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

        for (transform, mesh_renderer) in (&transform, &mesh_renderer).join() {
            let model_matrix = transform.model_matrix;

            let mesh_renderer = mesh_renderer as &MeshRenderer;

            let ref shader_data = mesh_renderer.material.shader_data;
//            let ref shader = mesh_renderer.material.shader;
            shader_data.bind_shader_uniforms(&model_matrix, &view_matrix, &projection_matrix);
//            gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, mesh.0.len() as i32));
            gl_call!(gl::DrawElements(gl::TRIANGLES,
                                  mesh_renderer.mesh.indices.len() as i32,
                                  gl::UNSIGNED_INT, 0 as *const c_void));
        }
    }
}