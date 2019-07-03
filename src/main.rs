extern crate glfw;
extern crate gl;

#[macro_use]
mod debugging;
mod shader_compilation;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint, WindowMode, Window, WindowEvent};
use std::ffi::CString;
use shader_compilation::Shader;
use shader_compilation::Program;
use std::sync::mpsc::Receiver;

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
    prog.use_program();

    gl_call!(gl::Viewport(0, 0, 800, 600));
    gl_call!(gl::ClearColor(1.0, 0.0, 0.0, 1.0));

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));
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