use std::os::raw::{c_uint, c_void};

#[derive(Debug)]
pub struct VAO {
    pub id: u32
}

impl VAO {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id));
        VAO { id }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindVertexArray(self.id));
        self
    }

    pub fn set_attribute(&self, attr: (u32, i32, c_uint, usize)) {
        let (index, count, gl_type, _type_size) = &attr;
        gl_call!(gl::VertexAttribPointer(*index, *count, *gl_type, gl::FALSE,
                                    0, 0 as *const c_void));
        gl_call!(gl::EnableVertexAttribArray(*index));
    }

    pub fn set_attributes(&self, attributes: &[(u32, i32, c_uint, usize)]) -> &Self {
        let stride = attributes.iter().fold(0, |mut sum, (_, count, _, size)| {
            sum += count * *size as i32;
            sum
        });

        let mut offset = 0;
        for (index, count, gl_type, type_size) in attributes {
            let total_size = 3 * *type_size;
            gl_call!(gl::VertexAttribPointer(*index, *count, *gl_type, gl::FALSE,
                                    stride,
                                    offset as *const c_void));
            gl_call!(gl::EnableVertexAttribArray(*index));
            offset += total_size;
        }
        self
    }
}