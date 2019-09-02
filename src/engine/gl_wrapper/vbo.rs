use std::os::raw::c_void;

#[derive(Debug)]
pub struct VertexAttribute {
    pub index: u32,
    pub components: u32,
}

#[derive(Debug)]
pub struct VBO {
    id: u32,
    pub(crate) attributes: Vec<VertexAttribute>
}

impl VBO {
    pub fn new(data: &[f32], attributes: Vec<VertexAttribute>) -> Self {
        // Unlink currently bound VAO
        gl_call!(gl::BindVertexArray(0));

        let mut id: u32 = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, id));
        gl_call!(gl::BufferData(gl::ARRAY_BUFFER,
                            (data.len() * std::mem::size_of::<f32>()) as isize,
                            data.as_ptr() as *const c_void,
                            gl::STATIC_DRAW));
        VBO { id, attributes }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.id));
        self
    }
}

// TODO implement destructor without breaking stuff
//impl Drop for VBO {
//    fn drop(&mut self) {
//        gl_call!(gl::DeleteBuffers(1, &self.id))
//    }
//}
