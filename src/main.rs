extern crate glfw;
extern crate gl;
#[macro_use]
extern crate specs_derive;

#[macro_use]
mod debugging;
mod gl_wrapper;
mod ecs;
mod shaders;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint, WindowMode, Window, WindowEvent, CursorMode};
use std::ffi::CString;
use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use gl_wrapper::vao::VAO;
use gl_wrapper::vbo::VBO;
use gl_wrapper::ebo::EBO;
use gl_wrapper::shader_compilation::*;
use gl_wrapper::texture_2d::Texture2D;
use ecs::components::*;
use specs::prelude::*;
use ecs::systems::*;
use ecs::resources::*;
use shaders::diffuse;
use crate::shaders::diffuse::DiffuseShader;
use crate::ecs::components::PointLight;
use glfw::ffi::glfwSwapInterval;
use nalgebra_glm::vec3;
use std::time::Duration;

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
    world.register::<Velocity>();
    world.register::<ecs::components::Material>();
    world.register::<Mesh>();
    world.register::<Camera>();
    world.insert(ActiveCamera::default());
    world.register::<DirLight>();
    world.register::<PointLight>();
    world.register::<Spotlight>();
    world.register::<Input>();
    world.insert(InputEventQueue::default());
    world.insert(InputCache::default());

    let (mut window, events) = setup_window("Window", 800, 800, glfw::WindowMode::Windowed);

    gl_call!(gl::Viewport(0, 0, 1000, 1000));
    gl_call!(gl::ClearColor(0.5, 0.8, 1.0, 1.0));

    let vertices = [
        -0.5f32, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, -1.0,
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, -1.0,
        0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, -1.0,
        0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, -1.0,
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, -1.0,
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, -1.0,
        -0.5, -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
        0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0,
        0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0,
        -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
        -0.5, -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
        -0.5, 0.5, 0.5, 1.0, 0.0, -1.0, 0.0, 0.0,
        -0.5, 0.5, -0.5, 1.0, 1.0, -1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 1.0, -1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 1.0, -1.0, 0.0, 0.0,
        -0.5, -0.5, 0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
        -0.5, 0.5, 0.5, 1.0, 0.0, -1.0, 0.0, 0.0,
        0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0,
        0.5, 0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
        0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0,
        0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
        0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 1.0, 0.0, -1.0, 0.0,
        0.5, -0.5, -0.5, 1.0, 1.0, 0.0, -1.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, -1.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, -1.0, 0.0,
        -0.5, -0.5, 0.5, 0.0, 0.0, 0.0, -1.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 1.0, 0.0, -1.0, 0.0,
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
        0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 0.0,
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
        -0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
    ].to_vec();

//    let indices = [
//        0u32, 1, 3,
//        1, 2, 3
//    ].to_vec();

    let vao = VAO::new();
    vao.bind();

    let vbo_vertices = VBO::new();
    vbo_vertices.bind().fill(&vertices);

//    let ebo = EBO::new();
//    ebo.bind().fill(&indices);

    vao.set_attributes(&[
        (0, 3, gl::FLOAT, std::mem::size_of::<f32>()),
        (1, 2, gl::FLOAT, std::mem::size_of::<f32>()),
        (2, 3, gl::FLOAT, std::mem::size_of::<f32>()),
    ]);

    let diffuse = Texture2D::new();
    diffuse.bind().fill("src/planks_oak.png");

    let specular = Texture2D::new();
    specular.bind().fill("src/specular.png");

    vao.bind();



    let transform_system = {
        let mut comps = world.write_storage::<Transform>();
        TransformSystem {
            reader_id: comps.register_reader(),
            dirty: BitSet::new(),
        }
    };

    let mut dispatcher = DispatcherBuilder::new()
        .with(MoveSystem, "move_system", &[])
        .with_barrier()
        .with(transform_system, "transform_system", &[])
        .with_thread_local(MeshRenderer)
        .build();




    for i in 0..10 {
        for j in 0..10 {
            let entity = world.create_entity()
                .with(Transform {
                    position: vec3(i as f32, -3.0, j as f32),
                    ..Transform::default()
                })
                .with(ecs::components::Material {
                    shader: Box::new(DiffuseShader::new(
                        diffuse.clone(),
                        specular.clone(),
                        vec3(1.0, 1.0, 1.0),
                        1.0,
                        0.0,
                        32.0))
                })
                .with(Mesh(vertices.clone()))
                .build();
        }
    }

    let entity = world.create_entity()
        .with(Transform {
            position: vec3(1.0f32, 1.0, 0.0),
            rotation: vec3(0.0f32, 1.0, 1.5),
            scale: vec3(0.5f32, 0.5, 0.5),
            ..Transform::default()
        })
        .with(Velocity(vec3(0.0, 0.0, -0.01)))
        .with(ecs::components::Material {
            shader: Box::new(DiffuseShader::new(
                diffuse,
                specular.clone(),
                vec3(1.0, 1.0, 1.0),
                1.0,
                0.0,
                32.0))
        })
        .with(Mesh(vertices))
        .build();

    use std::f32;
    let camera_entity = world.create_entity()
        .with(Transform {
            position: vec3(0.0, 0.0, 3.0),
            rotation: vec3(0.0, f32::consts::PI / 2.0 * 3.0, 1.0),
            ..Transform::default()
        })
//        .with(Velocity(vec3(0.0f32, 0.0, 0.0)))
        .with(Camera {
            fov: 60.0f32.to_radians(),
            aspect_ratio: 1.0,
            near_plane: 0.1,
            far_plane: 100.0,
        })
        .with(Input)
        .build();


    world.write_resource::<ActiveCamera>().entity = Some(camera_entity);


    let mut input_system = InputSystem;

    gl_call!(gl::Enable(gl::CULL_FACE));
    gl_call!(gl::CullFace(gl::BACK));
    gl_call!(gl::Enable(gl::DEPTH_TEST));
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


        dispatcher.dispatch(&mut world);
        input_system.run_now(&world);
        world.maintain();
//        gl_call!(gl::DrawElements(gl::TRIANGLES,
//                                  indices.len() as i32,
//                                  gl::UNSIGNED_INT, 0 as *const c_void));

        window.swap_buffers();
        window.glfw.poll_events();
        std::thread::sleep(Duration::new(0, 16666666));
    }
}
