use std::os::raw::c_void;
use crate::gl_wrapper::{BufferUpdateFrequency, NULLPTR};

#[derive(Debug)]
pub struct EBO {
    pub(crate) id: u32,
    length: usize
}

impl EBO {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::CreateBuffers(1, &mut id));
        EBO { id, length: 0 }
    }

    pub fn allocate(&mut self, elements: usize, update_frequency: BufferUpdateFrequency) {
        self.length = elements;
        gl_call!(gl::NamedBufferData(self.id,
            (elements * std::mem::size_of::<u32>()) as isize,
            NULLPTR,
            update_frequency.to_gl_enum()));
    }

    pub fn with(&mut self, data: &[u32], update_frequency: BufferUpdateFrequency) {
        self.length = data.len();
        gl_call!(gl::NamedBufferData(self.id,
            (data.len() * std::mem::size_of::<u32>()) as isize,
            data.as_ptr() as *mut c_void,
            update_frequency.to_gl_enum()));
    }

    pub fn update(&mut self, offset: usize, data: &[f32]) {
        gl_call!(gl::NamedBufferSubData(self.id,
            (offset * std::mem::size_of::<u32>()) as isize,
            (data.len() * std::mem::size_of::<u32>()) as isize,
            data.as_ptr() as *mut c_void))
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id));
        self
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

// TODO Implement proper destructor
//impl Drop for EBO {
//    fn drop(&mut self) {
//        gl_call!(gl::DeleteBuffers(1, &self.id));
//    }
//}