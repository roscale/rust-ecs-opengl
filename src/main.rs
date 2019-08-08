#![feature(const_fn)]
extern crate glfw;
extern crate gl;
#[macro_use]
extern crate specs_derive;
extern crate state;

#[macro_use]
mod debugging;
mod gl_wrapper;
mod ecs;
mod shaders;
mod containers;
mod utils;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint, WindowMode, Window, WindowEvent, CursorMode};
use std::sync::mpsc::Receiver;
use ecs::components::*;
use specs::prelude::*;
use ecs::systems::*;
use ecs::resources::*;
use crate::shaders::diffuse::DiffuseShader;
use crate::ecs::components::PointLight;
use glfw::ffi::{glfwSwapInterval, glfwGetTime};
use nalgebra_glm::{vec3, Mat3};
use containers::global_instances::*;
use crate::containers::global_instances::CONTAINER;
use crate::utils::ToVec3;
use nalgebra::{Vector, Matrix};
use ncollide3d::shape::{ShapeHandle, Cuboid};
use nphysics3d::object::{ColliderDesc, RigidBodyDesc, BodyStatus};
use nphysics3d::material::BasicMaterial;
use nphysics3d::algebra::Velocity3;

fn setup_window(title: &str, width: u32, height: u32, mode: WindowMode) -> (Window, Receiver<(f64, WindowEvent)>) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(width, height, title, mode).unwrap();
    window.set_key_polling(true);
    window.set_cursor_enter_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    window.set_cursor_pos(300.0, 300.0);
    window.set_raw_mouse_motion(true);

    window.make_current();
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe { glfwSwapInterval(0) };

    (window, events)
}

fn main() {
    let mut world = World::new();
    world.register::<Transform>();
    world.register::<MeshRenderer>();
    world.register::<Camera>();
    world.insert(ActiveCamera::default());
    world.register::<DirLight>();
    world.register::<PointLight>();
    world.register::<Spotlight>();
    world.register::<Input>();
    world.insert(InputEventQueue::default());
    world.insert(InputCache::default());

    // Physics stuff
    world.insert(PhysicsWorld {
        world: {
            let mut physics_world = nphysics3d::world::World::<f32>::new();
            physics_world.set_timestep(1.0f32 / 500.0f32);
            physics_world.set_gravity(Vector::y() * -9.81);
            physics_world
        },
        ..PhysicsWorld::default()
    });
    world.register::<RigidBody>();
    world.register::<BoxCollider>();
    world.register::<Collider>();

    CONTAINER.set_local(ModelLoader::default);
    CONTAINER.set_local(TextureCache::default);
    CONTAINER.set_local(DiffuseShader::default);


//    let shape = ShapeHandle::<f32>::new(Cuboid::new(0.5.to_vec3()));
//    let collider = ColliderDesc::new(shape)
//        .name("First box collider".to_owned());

//    let body_handle = RigidBodyDesc::new()
//        .mass(1.0)
//        .velocity(nphysics3d::algebra::Velocity3::linear(0.0, 50.0, 0.0))
//        .collider(&collider)
//        .build(&mut physics_world).handle();


    let (mut window, events) = setup_window("Window", 800, 800, glfw::WindowMode::Windowed);

    gl_call!(gl::Viewport(0, 0, 1000, 1000));
    gl_call!(gl::ClearColor(0.5, 0.8, 1.0, 1.0));

    let model_loader = CONTAINER.get_local::<ModelLoader>();
    let mesh_renderer = model_loader.load("models/cube/box_test.obj");

    let transform_system = {
        let mut comps = world.write_storage::<Transform>();
        TransformSystem {
            reader_id: comps.register_reader(),
            dirty: BitSet::new(),
        }
    };

    let sync_bodies_to_physics_system = {
        let mut transforms = world.write_storage::<Transform>();
        let mut rigidbodies = world.write_storage::<RigidBody>();
        SyncBodiesToPhysicsSystem {
            transforms_reader_id: transforms.register_reader(),
            rigidbodies_reader_id: rigidbodies.register_reader(),
        }
    };

    let sync_colliders_to_physics_system = {
        let mut colliders = world.write_storage::<Collider>();
        SyncCollidersToPhysicsSystem {
            colliders_reader_id: colliders.register_reader(),
        }
    };

//    let box_collider_system = {
//        let mut comps = world.write_storage::<BoxCollider>();
//        BoxColliderSystem {
//            reader_id: comps.register_reader(),
//            dirty: BitSet::new(),
//        }
//    };

    let mut dispatcher = DispatcherBuilder::new()
//        .with(box_collider_system, "box_collider_system", &[])
//        .with_barrier()
        // Physics
        .with(sync_bodies_to_physics_system, "sync_bodies_to_physics_system", &[])
        .with(sync_colliders_to_physics_system, "sync_colliders_to_physics_system", &[
            "sync_bodies_to_physics_system"
        ])

        .with(PhysicsStepperSystem, "physics_stepper", &[
            "sync_bodies_to_physics_system",
            "sync_colliders_to_physics_system"
        ])
        .with(SyncBodiesFromPhysicsSystem, "sync_bodies_from_physics_system", &[
            "physics_stepper"
        ])
        .with_barrier()
        .with(transform_system, "transform_system", &[])
        .with_thread_local(MeshRendererSystem)
        .build();
//    let (diffuse, specular) = {
//        let textures = CONTAINER.get_local::<TextureCache>();
//        (textures.get("src/planks_oak.png"),
//         textures.get("src/specular.png"))
//    };

//    for i in 0..10 {
//        for j in 0..10 {
//            let entity = world.create_entity()
//                .with(Transform {
//                    position: vec3(i as f32, -3.0, j as f32),
//                    scale: vec3(0.008, 0.008, 0.008),
//                    ..Transform::default()
//                })
//                .with(mesh_renderer.clone())
//                .build();
//        }
//    }

    use nalgebra::Point3;

    let floor = world.create_entity()
        .with(Transform {
            position: vec3(0.0, 0.0, 0.0),
            scale: 10.0.to_vec3(),
            ..Transform::default()
        })
        .with(mesh_renderer.clone())
        .with(RigidBody {
            status: BodyStatus::Static,
            ..RigidBody::default()
        })
        .with(Collider {
            shape: ShapeHandle::new(Cuboid::new(vec3(0.25, 0.25, 0.25) * 10.0)),
            material: BasicMaterial::default()
        })
        .build();


    let entity1 = world.create_entity()
        .with(Transform {
            position: vec3(0.0, 20.0, 0.0),
            ..Transform::default()
        })
        .with(mesh_renderer.clone())
        .with(RigidBody {
            mass: 1.0,
            angular_inertia: Mat3::identity(),
//            angular_inertia: 1.0.to_vec3(),
//            velocity: Velocity3 {
//                linear: vec3(5.0, 30.0, 0.0),
//                angular: vec3(1.0, 2.0, 3.0),
//            },
            ..RigidBody::default()
        })
        .with(Collider {
            shape: ShapeHandle::new(Cuboid::new(vec3(0.25, 0.25, 0.25))),
            material: BasicMaterial::new(0.5, 0.5)
        })
        .build();

    let entity2 = world.create_entity()
        .with(Transform {
            position: vec3(0.4, 10.0, 0.2),
            ..Transform::default()
        })
        .with(mesh_renderer.clone())
        .with(RigidBody {
            mass: 1.0,
            angular_inertia: Mat3::identity(),
//            velocity: Velocity3 {
//                linear: vec3(0.0, 20.0, 0.0),
//                angular: 0.0.to_vec3(),
//            },
            ..RigidBody::default()
        })
        .with(Collider {
            shape: ShapeHandle::new(Cuboid::new(vec3(0.25, 0.25, 0.25))),
            material: BasicMaterial::default()
        })
        .build();

    let light = world.create_entity()
        .with(Transform {
            position: vec3(10.0, 10.0, 10.0),
            scale: 0.1.to_vec3(),
            ..Transform::default()
        })
        .with(mesh_renderer.clone())
        .with(PointLight {
            color: 1.0.to_vec3(),
            range: 100.0,
            intensity: 1.0,
        })
        .with(RigidBody {
            velocity: Velocity3::linear(-1.0, 0.0, 0.0),
            status: BodyStatus::Kinematic,
            ..RigidBody::default()
        })
        .build();

    use std::f32;
    let camera_entity = world.create_entity()
        .with(Transform {
            position: vec3(0.0, 5.0, 3.0),
            rotation: vec3(0.0, f32::consts::PI / 2.0 * 3.0, 1.0),
            ..Transform::default()
        })
//        .with(Velocity(vec3(0.0f32, 0.0, 0.0)))
        .with(Camera {
            fov: 60.0f32.to_radians(),
            aspect_ratio: 1.0,
            near_plane: 0.1,
            far_plane: 10000.0,
        })
        .with(Input)
        .build();


    world.write_resource::<ActiveCamera>().entity = Some(camera_entity);

    let mut input_system = InputSystem;

    gl_call!(gl::Enable(gl::CULL_FACE));
    gl_call!(gl::CullFace(gl::BACK));
    gl_call!(gl::Enable(gl::DEPTH_TEST));

    let mut prev = 0.0;
    let mut frames = 0;

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {
                    world.write_resource::<InputEventQueue>().queue.push_back(event);
                }
            }
        };
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));


        dispatcher.dispatch(&world);
        input_system.run_now(&world);
        world.maintain();
//        {
//            let body = physics_world.rigid_body(body_handle);
//            println!("Cube physics pos: {}", body.unwrap().position());
//        }

        window.swap_buffers();
        window.glfw.poll_events();

        // Calculate framerate
        let now = unsafe { glfwGetTime() };
        frames += 1;
        let delta = now - prev;
        if delta >= 1.0 {
            prev = now;
            println!("Framerate: {}", f64::from(frames) / delta);
            frames = 0;
        }
    }
}
