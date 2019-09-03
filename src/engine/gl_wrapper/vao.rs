use std::os::raw::{c_uint, c_void};
use crate::gl_wrapper::vbo::{VBO, VertexAttribute};
use crate::gl_wrapper::ebo::EBO;

#[derive(Debug)]
pub struct VAO {
    id: u32
}

impl VAO {
    pub fn new(vbos: &[VBO], ebo: Option<&EBO>) -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::CreateVertexArrays(1, &mut id));
        gl_call!(gl::BindVertexArray(id));

        for vbo in vbos {
            vbo.bind();
            Self::set_attributes(&vbo.attributes);
        }
        if let Some(ebo) = ebo {
            ebo.bind();
        }
        gl_call!(gl::BindVertexArray(0));
        VAO { id }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindVertexArray(self.id));
        self
    }

    pub fn unbind(&self) -> &Self {
        gl_call!(gl::BindVertexArray(0));
        self
    }

    fn set_attributes(attributes: &[VertexAttribute]) {
        let stride = if attributes.len() == 1 {
            0 // Tightly packed, see OpenGL docs
        } else {
            attributes.iter().fold(0, |mut sum, VertexAttribute { index, components }| {
                sum += (*components as usize * std::mem::size_of::<f32>()) as i32;
                sum
            })
        };

        let mut offset = 0;
        for VertexAttribute { index, components} in attributes {
            let total_size = *components as usize * std::mem::size_of::<f32>();
            gl_call!(gl::VertexAttribPointer(*index, *components as i32, gl::FLOAT, gl::FALSE,
                                    stride,
                                    offset as *const c_void));
            gl_call!(gl::EnableVertexAttribArray(*index));
            offset += total_size;
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.id));
    }
}