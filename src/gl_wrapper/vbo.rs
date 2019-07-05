use std::os::raw::c_void;

#[derive(Debug)]
pub struct VBO {
    pub id: u32
}

impl VBO {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        VBO { id }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.id));
        self
    }

    pub fn fill(&self, vertices: &Vec<f32>) -> &Self {
        gl_call!(gl::BufferData(gl::ARRAY_BUFFER,
                            (vertices.len() * std::mem::size_of::<f32>()) as isize,
                            vertices.as_ptr() as *const c_void,
                            gl::STATIC_DRAW));
        self
    }
}