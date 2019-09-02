use std::os::raw::c_void;

#[derive(Debug)]
pub struct EBO {
    id: u32
}

impl EBO {
    pub fn new(indices: &[u32]) -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id));
        gl_call!(gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                            (indices.len() * std::mem::size_of::<u32>()) as isize,
                            indices.as_ptr() as *const c_void,
                            gl::STATIC_DRAW));
        EBO { id }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id));
        self
    }
}

// TODO Implement proper destructor
//impl Drop for EBO {
//    fn drop(&mut self) {
//        gl_call!(gl::DeleteBuffers(1, &self.id));
//    }
//}