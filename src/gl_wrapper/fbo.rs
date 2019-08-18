use crate::gl_wrapper::texture_2d::Texture2D;
use crate::gl_wrapper::rbo::RBO;

#[derive(Debug)]
pub struct FBO {
    pub id: u32
}

impl FBO {
    pub fn new() -> Self {
        let mut id = 0u32;
        gl_call!(gl::GenFramebuffers(1, &mut id));
        FBO { id }
    }

    pub fn bind(&self) {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, self.id));
    }

    pub fn bind_default() {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));
    }

    pub fn is_complete(&self) -> bool {
        gl_call!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER)) == gl::FRAMEBUFFER_COMPLETE
    }

    pub fn attach_color_texture(&self, texture: &Texture2D) {
        gl_call!(gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.id, 0));
    }

    pub fn attach_depth_stencil_texture(&self, texture: &Texture2D) {
        gl_call!(gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, texture.id, 0));
    }

    pub fn attach_depth_stencil_renderbuffer(&self, renderbuffer: &RBO) {
        gl_call!(gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, renderbuffer.id));
    }
}

impl Drop for FBO {
    fn drop(&mut self) {
        gl_call!(gl::DeleteFramebuffers(1, &self.id));
    }
}