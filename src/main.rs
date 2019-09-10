#![feature(const_fn)]
extern crate glfw;
extern crate gl;

extern crate state;
use engine::*;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint, WindowMode, Window, WindowEvent, CursorMode};
use std::sync::mpsc::Receiver;
use ecs::components::*;
use specs::prelude::*;
use ecs::systems::*;
use ecs::resources::*;
use crate::shaders::diffuse::DiffuseShader;
use crate::ecs::components::PointLight;
use glfw::ffi::{glfwSwapInterval};
use nalgebra_glm::{vec3, Mat3};
use crate::containers::*;
use nalgebra::{Vector};
use ncollide3d::shape::{ShapeHandle, Cuboid};
use nphysics3d::object::{BodyStatus};
use nphysics3d::material::BasicMaterial;
use nphysics3d::algebra::Velocity3;
use crate::shaders::outline::OutlineShader;
use crate::shaders::post_processing::{KernelShader, GaussianBlurShader};
use engine::shaders::cube_map::CubeMapShader;
use engine::gl_wrapper::texture_cube_map::TextureCubeMap;
use engine::shapes::PredefinedShapes;
use std::sync::Arc;
use debugging::debug_message_callback;
use std::os::raw::c_void;

fn setup_window(title: &str, width: u32, height: u32, mode: WindowMode) -> (Window, Receiver<(f64, WindowEvent)>) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersionMajor(4));
    glfw.window_hint(WindowHint::ContextVersionMinor(5));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlDebugContext(true));

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
    pretty_env_logger::init();

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
    world.insert(Time::default());
    world.register::<Outliner>();

    // Physics stuff
    world.insert(PhysicsWorld {
        world: {
            let mut physics_world = nphysics3d::world::World::<f32>::new();
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
    CONTAINER.set_local(CubeMapShader::default);
    CONTAINER.set_local(DiffuseShader::default);
    CONTAINER.set_local(OutlineShader::default);
    CONTAINER.set_local(KernelShader::default);
    CONTAINER.set_local(GaussianBlurShader::default);

    CONTAINER.set_local(PredefinedShapes::default);

    let (mut window, events) = setup_window("Window", 800, 800, glfw::WindowMode::Windowed);

    let model_loader = CONTAINER.get_local::<ModelLoader>();
    let mesh_renderer = model_loader.load("models/cube/box_test.obj");
//    let gun = model_loader.load("models/gun/modified_gun.obj");
    
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

    let physics_stepper = {
        let mut physics_world = world.write_resource::<PhysicsWorld>();
        PhysicsStepperSystem::new(&mut physics_world.world, 128)
    };

    let mut dispatcher = DispatcherBuilder::new()
        // Physics
        .with(sync_bodies_to_physics_system, "sync_bodies_to_physics_system", &[])
        .with(sync_colliders_to_physics_system, "sync_colliders_to_physics_system", &[
            "sync_bodies_to_physics_system"
        ])

        .with(physics_stepper, "physics_stepper", &[
            "sync_bodies_to_physics_system",
            "sync_colliders_to_physics_system"
        ])
        .with(SyncBodiesFromPhysicsSystem, "sync_bodies_from_physics_system", &[
            "physics_stepper"
        ])
        .with_barrier()
        .with(transform_system, "transform_system", &[])
        .with_thread_local(MeshRendererSystem::default())
        .build();


    // Scene objects & resources

    let skybox_texture = TextureCubeMap::new(&[
        "models/skybox/right.jpg",
        "models/skybox/left.jpg",
        "models/skybox/top.jpg",
        "models/skybox/bottom.jpg",
        "models/skybox/front.jpg",
        "models/skybox/back.jpg"
    ]);
    let skybox_texture = Arc::new(skybox_texture);

    let _floor = world.create_entity()
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
            material: BasicMaterial::default(),
        })
        .with(Outliner {
            scale: 1.05f32,
            color: vec3(1.0, 1.0, 0.0),
        })
        .build();

//    let _floor2 = world.create_entity()
//        .with(Transform {
//            position: vec3(-15.0, 0.0, 0.0),
//            scale: 3.0.to_vec3(),
//            ..Transform::default()
//        })
//        .with(gun.clone())
//        .with(Outliner {
//            scale: 1.05f32,
//            color: vec3(0.0, 1.0, 1.0),
//        })
//        .build();


    let _entity1 = world.create_entity()
        .with(Transform {
            position: vec3(0.0, 20.0, 0.0),
            ..Transform::default()
        })
        .with(mesh_renderer.clone())
        .with(RigidBody {
            mass: 1.0,
            angular_inertia: Mat3::identity(),
            ..RigidBody::default()
        })
        .with(Collider {
            shape: ShapeHandle::new(Cuboid::new(vec3(0.25, 0.25, 0.25))),
            material: BasicMaterial::new(0.5, 0.5),
        })
        .build();

    let _entity2 = world.create_entity()
        .with(Transform {
            position: vec3(0.4, 10.0, 0.2),
            ..Transform::default()
        })
        .with(mesh_renderer.clone())
        .with(RigidBody {
            mass: 1.0,
            angular_inertia: Mat3::identity(),
            ..RigidBody::default()
        })
        .with(Collider {
            shape: ShapeHandle::new(Cuboid::new(vec3(0.25, 0.25, 0.25))),
            material: BasicMaterial::default(),
        })
        .build();

    let _light = world.create_entity()
        .with(Transform {
            position: vec3(10.0, 10.0, 10.0),
//            scale: 0.1.to_vec3(),
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
        .with(Camera::new(70.0f32.to_radians(),
                          1.0,
                          0.1,
                          1000.0,
                          Background::Skybox(skybox_texture),
                          vec![
//                Box::new(Kernel::new(vec![
//                    1.0, 1.0, 1.0,
//                    1.0, -8.0, 1.0,
//                    1.0, 1.0, 1.0
//                ])),
//                Box::new(GaussianBlur::new(vec![0.034619, 0.044859, 0.055857, 0.066833, 0.076841, 0.084894, 0.090126, 0.09194, 0.090126, 0.084894, 0.076841, 0.066833, 0.055857, 0.044859, 0.034619])),
            ]))
        .with(Input)
        .build();


    world.write_resource::<ActiveCamera>().entity = Some(camera_entity);

    let mut input_system = InputSystem;
    let mut print_framerate = PrintFramerate::default();

    gl_call!(gl::Enable(gl::DEBUG_OUTPUT));
    gl_call!(gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
    gl_call!(gl::DebugMessageCallback(debug_message_callback, 0 as *const c_void));
    gl_call!(gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, 0 as *const u32, gl::TRUE));

    gl_call!(gl::Enable(gl::CULL_FACE));
    gl_call!(gl::CullFace(gl::BACK));
    gl_call!(gl::Enable(gl::DEPTH_TEST));
    gl_call!(gl::Enable(gl::STENCIL_TEST));

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {
                    world.write_resource::<InputEventQueue>().queue.push_back(event);
                }
            };
        };

        dispatcher.dispatch(&world);
        input_system.run_now(&world);
        world.maintain();

        window.swap_buffers();
        window.glfw.poll_events();

        world.write_resource::<Time>().tick();
        print_framerate.run_now(&world);
    }
}
