use std::os::raw::c_void;

#[derive(Debug)]
pub struct EBO {
    pub id: u32
}

impl EBO {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        EBO { id }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id));
        self
    }

    pub fn fill(&self, indices: &[u32]) -> &Self {
        gl_call!(gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                            (indices.len() * std::mem::size_of::<u32>()) as isize,
                            indices.as_ptr() as *const c_void,
                            gl::STATIC_DRAW));
        self
    }
}