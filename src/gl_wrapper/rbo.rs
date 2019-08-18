pub struct RBO {
    pub id: u32
}

impl RBO {
    pub fn new() -> Self {
        let mut id = 0u32;
        gl_call!(gl::GenRenderbuffers(1, &mut id));
        RBO { id }
    }

    pub fn bind(&self) {
        gl_call!(gl::BindRenderbuffer(gl::RENDERBUFFER, self.id));
    }

    pub fn create_depth_stencil(&self, width: i32, height: i32) {
        gl_call!(gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height));
    }
}