extern crate glfw;
extern crate gl;

#[macro_use]
mod debugging;
mod gl_wrapper;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint, WindowMode, Window, WindowEvent};
use std::ffi::CString;
use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::VBO;
use crate::gl_wrapper::ebo::EBO;
use crate::gl_wrapper::shader_compilation::*;
use glfw::ffi::glfwGetTime;

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
    let (mut window, events) = setup_window("Window", 800, 600, glfw::WindowMode::Windowed);

    let vert_shader = Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let prog = Program::from_shaders(vert_shader, frag_shader).unwrap();

    gl_call!(gl::Viewport(0, 0, 800, 600));
    gl_call!(gl::ClearColor(0.2, 0.3, 0.3, 1.0));

    let vertices = [
        0.5f32, 0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        -0.5, 0.5, 0.0, 0.0, 0.0, 1.0
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

    vao.set_attributes(&[(3, gl::FLOAT, std::mem::size_of::<f32>()),
        (3, gl::FLOAT, std::mem::size_of::<f32>())]);

    prog.use_program();
    vao.bind();

    let mut offset = 0f32;

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        };
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));
        gl_call!(gl::DrawElements(gl::TRIANGLES,
                                  indices.len() as i32,
                                  gl::UNSIGNED_INT, 0 as *const c_void));
        window.swap_buffers();
        window.glfw.poll_events();

        offset += 0.001;
        prog.set_uniform1f("offset", offset);
//        unsafe {
//            let time: f32 = (glfwGetTime().sin() as f32 + 1f32) / 2f32;
//            prog.set_uniform("inputColor", &[0.0, time, 0.0, 1.0]);
//        }
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