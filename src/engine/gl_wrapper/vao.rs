use crate::gl_wrapper::vbo::{VBO, VertexAttribute};
use crate::gl_wrapper::ebo::EBO;

#[derive(Debug)]
pub struct VAO {
    id: u32
}

impl VAO {
    pub fn new(vbos: &[VBO], ebo: Option<&EBO>) -> Self {
        let mut vao = VAO { id: 0 };
        gl_call!(gl::CreateVertexArrays(1, &mut vao.id));

        for (binding_index, vbo) in vbos.iter().enumerate() {
            let binding_index = binding_index as u32;
            vao.set_attributes(binding_index, &vbo);
        }
        if let Some(ebo) = ebo {
            gl_call!(gl::VertexArrayElementBuffer(vao.id, ebo.id));
        }
        vao
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindVertexArray(self.id));
        self
    }

    pub fn unbind(&self) -> &Self {
        gl_call!(gl::BindVertexArray(0));
        self
    }

    fn set_attributes(&self, binding_index: u32, vbo: &VBO) {
        // FIXME Fix multiple vertex attributes per buffer with DSA
//        let stride = if attributes.len() == 1 {
//            0 // Tightly packed, see OpenGL docs
//        } else {
//            attributes.iter().fold(0, |mut sum, VertexAttribute { index, components }| {
//                sum += (*components as usize * std::mem::size_of::<f32>()) as i32;
//                sum
//            })
//        };
//        gl_call!(gl::VertexAttribPointer(*index, *components as i32, gl::FLOAT, gl::FALSE,
//                                    stride,
//                                    offset as *const c_void));

        let mut offset = 0;
        for VertexAttribute { index, components} in &vbo.attributes {
            gl_call!(gl::EnableVertexArrayAttrib(self.id, *index));
            gl_call!(gl::VertexArrayAttribFormat(self.id, *index, *components as i32, gl::FLOAT, gl::FALSE, offset as u32));
            gl_call!(gl::VertexArrayAttribBinding(self.id, *index, binding_index));

            let total_size = (*components as usize) * std::mem::size_of::<f32>();
            offset += total_size;
            gl_call!(gl::VertexArrayVertexBuffer(self.id, binding_index, vbo.id, 0, ((*components as usize) * std::mem::size_of::<f32>()) as i32));
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.id));
    }
}