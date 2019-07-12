extern crate glfw;
extern crate gl;

#[macro_use]
mod debugging;
mod gl_wrapper;
mod ecs;
mod shaders;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint, WindowMode, Window, WindowEvent};
use std::ffi::CString;
use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use gl_wrapper::vao::VAO;
use gl_wrapper::vbo::VBO;
use gl_wrapper::ebo::EBO;
use gl_wrapper::shader_compilation::*;
use gl_wrapper::texture_2d::Texture2D;
use ecs::components::{Transform, Velocity, Mesh, Camera};
use specs::prelude::*;
use ecs::systems::{MoveSystem, MeshRenderer};
use cgmath::{vec3, Vector3};
use ecs::resources::ActiveCamera;
use shaders::diffuse;
use crate::shaders::diffuse::DiffuseShader;

fn setup_window(title: &str, width: u32, height: u32, mode: WindowMode) -> (Window, Receiver<(f64, WindowEvent)>) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(width, height, title, mode).unwrap();
    window.set_key_polling(true);
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s) as *const _);
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

    let (mut window, events) = setup_window("Window", 800, 800, glfw::WindowMode::Windowed);

    gl_call!(gl::Viewport(0, 0, 800, 800));
    gl_call!(gl::ClearColor(0.2, 0.3, 0.3, 1.0));

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

    let texture = Texture2D::new();
    texture.bind().fill("src/wall.jpg");

    vao.bind();

    let entity = world.create_entity()
        .with(Transform {
            position: vec3(0.0f32, 0.0, 5.0),
            rotation: vec3(2.2f32, 0.9, 1.7),
            scale: vec3(1.0f32, 1.0, 1.0),
        })
        .with(Velocity(vec3(0.0f32, 0.0, -0.01)))
        .with(ecs::components::Material {
            shader: Box::new(DiffuseShader::new(
                texture.clone(),
                vec3(1.0, 1.0, 1.0),
                0.2,
                0.5,
                32.0))
        })
        .with(Mesh(vertices.clone()))
        .build();

    let entity = world.create_entity()
        .with(Transform {
            position: vec3(1.0f32, 1.0, 0.0),
            rotation: vec3(0.0f32, 1.0, 1.5),
            scale: vec3(0.5f32, 0.5, 0.5),
        })
        .with(ecs::components::Material {
            shader: Box::new(DiffuseShader::new(
                texture,
                vec3(1.0, 1.0, 1.0),
                0.2,
                0.5,
                32.0))
        })
        .with(Mesh(vertices))
        .build();

    use std::f32;
    let camera_entity = world.create_entity()
        .with(Transform {
            position: vec3(0.0, 0.0, 3.0),
            rotation: vec3(0.0, f32::consts::PI / 2.0 * 3.0, 1.0),
            scale: vec3(1.0, 1.0, 1.0),
        })
        .with(Velocity(vec3(0.0f32, 0.0, 0.0)))
        .with(Camera {
            fov: 60.0,
            aspect_ratio: 1.0,
            near_plane: 0.1,
            far_plane: 100.0,
        }).build();

    world.write_resource::<ActiveCamera>().entity = Some(camera_entity);

    let mut dispatcher = DispatcherBuilder::new()
        .with(MoveSystem, "move_system", &[])
        .with_thread_local(MeshRenderer)
        .build();

    gl_call!(gl::Enable(gl::DEPTH_TEST));
    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        };
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));


        dispatcher.dispatch(&mut world);
//        gl_call!(gl::DrawElements(gl::TRIANGLES,
//                                  indices.len() as i32,
//                                  gl::UNSIGNED_INT, 0 as *const c_void));

        window.swap_buffers();
        window.glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}