extern crate glfw;
extern crate gl;

#[macro_use]
mod debugging;
mod gl_wrapper;
mod ecs;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint, WindowMode, Window, WindowEvent};
use std::ffi::CString;
use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::VBO;
use crate::gl_wrapper::ebo::EBO;
use crate::gl_wrapper::shader_compilation::*;
use crate::gl_wrapper::texture_2d::Texture2D;
use crate::ecs::components::{Position, Velocity, Rotation, Scale};
use specs::prelude::*;
use crate::ecs::systems::{MoveSystem, MeshRenderer};

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
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Rotation>();
    world.register::<Scale>();
    world.register::<ecs::components::Shader>();

    let (mut window, events) = setup_window("Window", 800, 600, glfw::WindowMode::Windowed);

    let vert_shader = Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    gl_call!(gl::Viewport(0, 0, 800, 800));
    gl_call!(gl::ClearColor(0.2, 0.3, 0.3, 1.0));

    let vertices = [
        0.5f32, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        -0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0
    ].to_vec();

    let indices = [
        0u32, 1, 3,
        1, 2, 3
    ].to_vec();


    let vao = VAO::new();
    vao.bind();

    let vbo = VBO::new();
    vbo.bind().fill(&vertices);

    let ebo = EBO::new();
    ebo.bind().fill(&indices);

    vao.set_attributes(&[
        (3, gl::FLOAT, std::mem::size_of::<f32>()),
        (3, gl::FLOAT, std::mem::size_of::<f32>()),
        (2, gl::FLOAT, std::mem::size_of::<f32>()),
    ]);

    gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32));
    gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32));

    let texture = Texture2D::new();
    texture.bind().fill("src/wall.jpg");

    let prog = Program::from_shaders(vert_shader, frag_shader).unwrap();


    Texture2D::activate(0);
    texture.bind();

    vao.bind();

    let entity = world.create_entity()
        .with(Position((0.0f32, 0.0, 0.0).into()))
        .with(Rotation((0.0f32, 0.0, 1.0).into()))
        .with(Scale((1.5f32, 1.0, 1.0).into()))
//        .with(Velocity((0.01f32, 0.0, 0.0).into()))
        .with(ecs::components::Shader(prog))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(MoveSystem, "move_system", &[])
        .with_thread_local(MeshRenderer)
        .build();

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        };
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));


        dispatcher.dispatch(&mut world);
        gl_call!(gl::DrawElements(gl::TRIANGLES,
                                  indices.len() as i32,
                                  gl::UNSIGNED_INT, 0 as *const c_void));

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