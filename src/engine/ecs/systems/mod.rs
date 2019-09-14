mod physics;

pub use physics::*;

use specs::prelude::*;
use specs::{System, WriteStorage, ReadStorage};
use crate::ecs::components::*;
use crate::ecs::resources::*;
use nalgebra_glm::{vec2, Mat4, vec3};
use nalgebra::{Vector3, Matrix4};
use glfw::{Key, WindowEvent};
use ncollide3d::shape::{ShapeHandle, Cuboid};
use nphysics3d::object::ColliderDesc;
use nphysics3d::material::MaterialHandle;
use glfw::ffi::glfwGetTime;
use crate::shaders::outline::OutlineData;
use crate::shaders::ShaderData;
use crate::gl_wrapper::fbo::FBO;
use crate::containers::CONTAINER;
use crate::shapes::PredefinedShapes;
use crate::shaders::cube_map::CubeMapShader;
use crate::gl_wrapper::texture_cube_map::TextureCubeMap;
use crate::gl_wrapper::ubo::{Std140, GlslTypes, UBO, ComputeStd140LayoutSize};
use std::os::raw::c_void;
use crate::gl_wrapper::BufferUpdateFrequency;

struct CameraUBO<'a> {
    pub view: &'a Mat4,
    pub projection: &'a Mat4,
}

impl<'a> Std140 for CameraUBO<'a> {
    fn get_std140_layout(&self) -> &'static [GlslTypes] {
        &[
            GlslTypes::Mat4,
            GlslTypes::Mat4
        ]
    }

    fn write_to_ubo(&self, ubo: &UBO) {
        unsafe {
//            gl_call!(gl::NamedBufferSubData(ubo.id, 0, 4 * 16, self.view.as_ptr() as *mut c_void));
//            gl_call!(gl::NamedBufferSubData(ubo.id, 4 * 16, 4 * 16, self.projection.as_ptr() as *mut c_void));
            let buf: *mut c_void = gl_call!(gl::MapNamedBufferRange(ubo.id, 0, ubo.layout.compute_std140_layout_size() as isize, gl::MAP_WRITE_BIT | gl::MAP_UNSYNCHRONIZED_BIT));
            let buf = buf.cast::<f32>();
            self.view.as_ptr().copy_to_nonoverlapping(buf, 16);
            self.projection.as_ptr().copy_to_nonoverlapping(buf.offset(16), 16);
            gl_call!(gl::UnmapNamedBuffer(ubo.id));
        }
    }
}

pub struct TransformSystem {
    pub reader_id: ReaderId<ComponentEvent>,
    pub dirty: BitSet,
}

impl<'a> System<'a> for TransformSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Transform>);

    fn run(&mut self, (_entities, mut transforms): Self::SystemData) {
        self.dirty.clear();
        let events = transforms.channel().read(&mut self.reader_id);

        for event in events {
            match event {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    self.dirty.add(*id);
                }
                ComponentEvent::Removed(_) => (),
            }
        }

        for (mut transforms, _entity) in (&mut transforms.restrict_mut(), &self.dirty).join() {
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

    fn run(&mut self, (mut input_event_queue, _inputs, mut transforms, cameras, active_camera, mut input_cache): Self::SystemData) {
        let active_camera = active_camera.entity.unwrap();
        let _camera = cameras.get(active_camera).unwrap();

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

//                    transform.rotation.y += x * 0.001;
//                    transform.rotation.x -= y * 0.001;
                }

                WindowEvent::Key(key, _, action, _) => {
                    input_cache.key_states.insert(*key, *action);
                }

                _ => {}
            }
        }

        if input_cache.is_key_pressed(Key::W) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position += transform.forward().scale(0.03f32);
        }

        if input_cache.is_key_pressed(Key::S) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position -= transform.forward().scale(0.03f32);
        }

        if input_cache.is_key_pressed(Key::A) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position -= transform.forward().cross(&Vector3::y()).scale(0.03f32);
        }

        if input_cache.is_key_pressed(Key::D) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position += transform.forward().cross(&Vector3::y()).scale(0.03f32);
        }

        if input_cache.is_key_pressed(Key::Q) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position.y += 0.03;
        }

        if input_cache.is_key_pressed(Key::Z) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position.y -= 0.03;
        }
    }
}

pub struct MeshRendererSystem {
    camera_matrices_ubo: UBO
}

impl Default for MeshRendererSystem {
    fn default() -> Self {
        let camera_matrices_ubo = UBO::new(&[
            GlslTypes::Mat4,
            GlslTypes::Mat4,
        ], BufferUpdateFrequency::Often);

        camera_matrices_ubo.bind(0);
        MeshRendererSystem { camera_matrices_ubo }
    }
}



impl<'a> System<'a> for MeshRendererSystem {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Transform>,
                       ReadStorage<'a, MeshRenderer>,
                       ReadStorage<'a, Camera>,
                       Read<'a, ActiveCamera>,
                       ReadStorage<'a, PointLight>,
                       ReadStorage<'a, Outliner>);

    fn run(&mut self, (entities, transforms, mesh_renderer, camera, active_camera, point_lights, outliners): Self::SystemData) {
        let (camera, cam_tr) = match active_camera.entity {
            Some(e) => (
                camera.get(e).expect("Active camera must have a Camera component"),
                transforms.get(e).expect("Active camera must have a Transform component")
            ),
            None => return
        };

        let direction = vec3(
            cam_tr.rotation.x.cos() * cam_tr.rotation.y.cos(),
            cam_tr.rotation.x.sin(),
            cam_tr.rotation.x.cos() * cam_tr.rotation.y.sin(),
        );

        let view_matrix = nalgebra_glm::look_at(&cam_tr.position, &(cam_tr.position + direction), &Vector3::y());

        let projection_matrix = match camera.projection {
            Projection::Orthographic(size) => {
                nalgebra_glm::ortho(-camera.aspect_ratio * size, camera.aspect_ratio * size, -size, size, camera.near_plane, camera.far_plane)
            }
            Projection::Perspective(fov) => {
                nalgebra_glm::perspective(camera.aspect_ratio, fov, camera.near_plane, camera.far_plane)
            }
        };

        {
            let camera_ubo_struct = CameraUBO { view: &view_matrix, projection: &projection_matrix };
            self.camera_matrices_ubo.update(&camera_ubo_struct);
        }


        // Post processing
        if camera.post_processing_effects.is_empty() {
            FBO::bind_default();
        } else {
            camera.fb.bind();
        }

        gl_call!(gl::Viewport(0, 0, 800, 800));
        gl_call!(gl::Enable(gl::DEPTH_TEST));
        gl_call!(gl::DepthFunc(gl::LESS));
        gl_call!(gl::Enable(gl::STENCIL_TEST));
        gl_call!(gl::StencilMask(0xFF));

        if let Background::Color(r, g, b) = camera.background {
            gl_call!(gl::ClearColor(r, g, b, 1.0));
        }

        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT));

        gl_call!(gl::StencilFunc(gl::ALWAYS, 1, 0xFF));
        for (entity, transform, mesh_renderer) in (&entities, &transforms, &mesh_renderer).join() {
            let model_matrix = transform.model_matrix;

            let mesh_renderer = mesh_renderer as &MeshRenderer;
            mesh_renderer.mesh.vao.bind();

            let shader_data = &mesh_renderer.material.shader_data;
            shader_data.bind_model(&model_matrix);
            shader_data.bind_lights(&transforms, &point_lights);

            // Outline stencil test
            if outliners.get(entity).is_some() {
                gl_call!(gl::StencilMask(0xFF));
                gl_call!(gl::StencilOp(gl::REPLACE, gl::REPLACE, gl::REPLACE));
            } else {
                // Disable stencil write
                gl_call!(gl::StencilMask(0x00));
                gl_call!(gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP));
            }

            gl_call!(gl::DrawElements(gl::TRIANGLES,
                                  mesh_renderer.mesh.indices.len() as i32,
                                  gl::UNSIGNED_INT, std::ptr::null()));
        }

        // Draw skybox
        if let Background::Skybox(texture) = &camera.background {
            // Disable stencil write
            gl_call!(gl::StencilMask(0x00));
            gl_call!(gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP));

            gl_call!(gl::DepthFunc(gl::LEQUAL));

            let cubemap_shader = CONTAINER.get_local::<CubeMapShader>();
            cubemap_shader.bind();

            let cube_vao = CONTAINER.get_local::<PredefinedShapes>().shapes.get("unit_cube").unwrap();
            cube_vao.bind();
            TextureCubeMap::activate(0);
            texture.bind();

            gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, 36));
        }

        // Draw outlined objects
        gl_call!(gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF));
        gl_call!(gl::StencilMask(0x00));
        gl_call!(gl::Disable(gl::DEPTH_TEST));

        for (transform, mesh_renderer, outliner) in (&transforms, &mesh_renderer, &outliners).join() {
            // Calculate scaled model matrix
            let scaled_model_matrix = {
                let translate_matrix = Matrix4::new_translation(&transform.position);

                let rotate_matrix = Matrix4::from_euler_angles(
                    transform.rotation.x,
                    transform.rotation.y,
                    transform.rotation.z,
                );

                let scale_vec = transform.scale * outliner.scale;
                let scale_matrix: Mat4 = Matrix4::new_nonuniform_scaling(&scale_vec);
                translate_matrix * rotate_matrix * scale_matrix
            };

            let mesh_renderer = mesh_renderer as &MeshRenderer;
            mesh_renderer.mesh.vao.bind();

            let shader_data = OutlineData {
                color: outliner.color
            };

            shader_data.bind_model(&scaled_model_matrix);
            gl_call!(gl::DrawElements(gl::TRIANGLES,
                                  mesh_renderer.mesh.indices.len() as i32,
                                  gl::UNSIGNED_INT, std::ptr::null()));
        }

        if !camera.post_processing_effects.is_empty() {
            let mut last_fb = &camera.fb;
            for i in 0..camera.post_processing_effects.len() - 1 {
                last_fb = camera.post_processing_effects[i].apply(last_fb);
            }
            camera.post_processing_effects.last().unwrap().apply_to_screen(last_fb);
        }
    }
}

//////////////////
// Physics systems
//////////////////

pub struct BoxColliderSystem {
    pub reader_id: ReaderId<ComponentEvent>,
    pub dirty: BitSet,
}

impl<'a> System<'a> for BoxColliderSystem {
    type SystemData = (ReadStorage<'a, Transform>,
                       Write<'a, PhysicsWorld>,
                       ReadStorage<'a, BoxCollider>);

    fn run(&mut self, (transforms, mut physics_world, box_colliders): Self::SystemData) {
        let mut inserted = BitSet::new();
        let events = box_colliders.channel().read(&mut self.reader_id);

        for event in events {
            match event {
                ComponentEvent::Inserted(id) => {
                    inserted.add(*id);
                }
                ComponentEvent::Modified(id) => {
                    self.dirty.add(*id);
                }
                ComponentEvent::Removed(_) => (),
            }
        }

        for (transform, box_collider, _entity) in (&transforms, &box_colliders, &inserted).join() {
            let transform = transform as &Transform;
            let box_collider = box_collider as &BoxCollider;

            let half_size = box_collider.box_size.scale(0.5);
            let shape = ShapeHandle::<f32>::new(Cuboid::new(half_size));
            let _collider = ColliderDesc::new(shape)
                .translation(transform.position)
                .rotation(transform.rotation)
                .material(MaterialHandle::new(box_collider.material))
                .build(&mut physics_world.world);
        }
    }
}

#[derive(Default)]
pub struct PrintFramerate {
    prev: f64,
    frames: u32,
}

impl<'a> System<'a> for PrintFramerate {
    type SystemData = Read<'a, Time>;

    fn run(&mut self, _time: Self::SystemData) {
        self.frames += 1;
        let now = unsafe { glfwGetTime() };
        let delta = now - self.prev;
        if delta >= 1.0 {
            self.prev = now;
            println!("Framerate: {}", f64::from(self.frames) / delta);
            self.frames = 0;
        }
    }
}