mod physics;

pub use physics::*;

use specs::prelude::*;
use specs::{System, WriteStorage, ReadStorage};
use crate::ecs::components::*;
use crate::ecs::resources::*;
use nalgebra_glm::{vec2, Mat4, vec3};
use crate::gl_wrapper::fbo::FBO;
use crate::gl_wrapper::rbo::RBO;
use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::VBO;
use crate::gl_wrapper::texture_2d::Texture2D;
use crate::containers::global_instances::CONTAINER;
use crate::shaders::post_processing::PostProcessingShader;

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
//            println!("EVENT: {:#?}", *event);
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
            transform.position += transform.forward().scale(0.01f32);
        }

        if input_cache.is_key_pressed(Key::S) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position -= transform.forward().scale(0.01f32);
        }

        if input_cache.is_key_pressed(Key::A) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position -= transform.forward().cross(&Vector3::y()).scale(0.01f32);
        }

        if input_cache.is_key_pressed(Key::D) {
            let transform = transforms.get_mut(active_camera).unwrap();
            transform.position += transform.forward().cross(&Vector3::y()).scale(0.01f32);
        }
    }
}

pub struct MeshRendererSystem {
    pp_fb: FBO,
    pp_color_texture: Texture2D,
    quad_vao: VAO,
}

// TODO Move framebuffer into camera
impl MeshRendererSystem {
    pub fn new() -> Self {
        // Post processing
        let pp_color_texture = Texture2D::new();
        pp_color_texture.bind();
        pp_color_texture.allocate_color(800, 800);

        let depth_stencil_rb = RBO::new();
        depth_stencil_rb.bind();
        depth_stencil_rb.create_depth_stencil(800, 800);

        let pp_fb = FBO::new();
        pp_fb.bind();
        pp_fb.attach_color_texture(&pp_color_texture);
        pp_fb.attach_depth_stencil_renderbuffer(&depth_stencil_rb);
        FBO::bind_default();

        // All screen quad
        let quad_pos = [
            -1.0f32, 1.0,
            -1.0, -1.0,
            1.0, -1.0,
            -1.0, 1.0,
            1.0, -1.0,
            1.0, 1.0,
        ];

        let quad_tex = [
            0.0f32, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 0.0,
            1.0, 1.0
        ];

        let quad_vao = VAO::new();
        quad_vao.bind();

        let quad_pos_vbo = VBO::new();
        quad_pos_vbo.bind();
        quad_pos_vbo.fill(&quad_pos);
        quad_vao.set_attribute((0, 2, gl::FLOAT, std::mem::size_of::<f32>()));

        let quad_tex_vbo = VBO::new();
        quad_tex_vbo.bind();
        quad_tex_vbo.fill(&quad_tex);
        quad_vao.set_attribute((1, 2, gl::FLOAT, std::mem::size_of::<f32>()));

        MeshRendererSystem {
            pp_fb,
            pp_color_texture,
            quad_vao
        }
    }
}

use nalgebra::{Vector3, Matrix4};
use glfw::{Key, WindowEvent};
use ncollide3d::shape::{ShapeHandle, Cuboid};
use nphysics3d::object::ColliderDesc;
use nphysics3d::material::MaterialHandle;
use glfw::ffi::glfwGetTime;
use crate::shaders::outline::OutlineData;
use crate::shaders::ShaderData;

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

        let look_at = nalgebra_glm::look_at(&cam_tr.position, &(cam_tr.position + direction), &Vector3::y());
        let view_matrix = look_at.prepend_translation(&(-cam_tr.position));

        let projection_matrix = nalgebra_glm::perspective(1f32, camera.fov, camera.near_plane, camera.far_plane);

        // Post processing
        self.pp_fb.bind();

        gl_call!(gl::Viewport(0, 0, 800, 800));
        gl_call!(gl::Enable(gl::DEPTH_TEST));
        gl_call!(gl::StencilMask(0xFF));
        gl_call!(gl::ClearColor(0.5, 0.8, 1.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT));

        gl_call!(gl::StencilFunc(gl::ALWAYS, 1, 0xFF));
        for (entity, transform, mesh_renderer) in (&entities, &transforms, &mesh_renderer).join() {
            let model_matrix = transform.model_matrix;

            let mesh_renderer = mesh_renderer as &MeshRenderer;
            mesh_renderer.mesh.vao.bind();

            let shader_data = &mesh_renderer.material.shader_data;
            shader_data.bind_mvp(&model_matrix, &view_matrix, &projection_matrix, &cam_tr.position);
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

            let shader_data = OutlineData {
                color: outliner.color
            };

            shader_data.bind_mvp(&scaled_model_matrix, &view_matrix, &projection_matrix, &cam_tr.position);
            gl_call!(gl::DrawElements(gl::TRIANGLES,
                                  mesh_renderer.mesh.indices.len() as i32,
                                  gl::UNSIGNED_INT, std::ptr::null()));
        }

        // Post processing
        FBO::bind_default();
        gl_call!(gl::Viewport(0, 0, 800, 800));
        gl_call!(gl::ClearColor(1.0, 0.5, 1.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT));

        let pp_shader = CONTAINER.get_local::<PostProcessingShader>();
        pp_shader.bind_screen_texture(&self.pp_color_texture);
        self.quad_vao.bind();
        gl_call!(gl::Disable(gl::DEPTH_TEST));
        gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, 6));
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

        for (transform, box_collider, entity) in (&transforms, &box_colliders, &inserted).join() {
            let transform = transform as &Transform;
            let box_collider = box_collider as &BoxCollider;

            let half_size = box_collider.box_size.scale(0.5);
            let shape = ShapeHandle::<f32>::new(Cuboid::new(half_size));
            let collider = ColliderDesc::new(shape)
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

    fn run(&mut self, time: Self::SystemData) {
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