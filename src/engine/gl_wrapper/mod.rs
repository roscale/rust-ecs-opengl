pub mod shader_compilation;
pub mod vao;
pub mod vbo;
pub mod ebo;
pub mod texture_2d;
pub mod texture_cube_map;
pub mod fbo;
pub mod rbo;
pub mod ubo;

pub use shader_compilation::*;
pub use vao::*;
pub use vbo::*;
pub use ebo::*;
pub use texture_2d::*;
pub use texture_cube_map::*;
pub use fbo::*;
pub use rbo::*;
pub use ubo::*;
use std::ffi::c_void;

pub const NULLPTR: *mut c_void = 0 as *mut c_void;

pub enum BufferUpdateFrequency {
    Never,
    Occasionally,
    Often,
}

impl BufferUpdateFrequency {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            BufferUpdateFrequency::Never => gl::STATIC_DRAW,
            BufferUpdateFrequency::Occasionally => gl::DYNAMIC_DRAW,
            BufferUpdateFrequency::Often => gl::STREAM_DRAW,
        }
    }
}