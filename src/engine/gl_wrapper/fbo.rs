use crate::gl_wrapper::texture_2d::Texture2D;
use crate::gl_wrapper::rbo::RBO;

#[derive(Debug)]
pub enum DepthStencilTarget {
    Texture2D(Texture2D),
    RBO(RBO)
}

#[derive(Debug)]
pub struct FBO {
    id: u32,
    pub(crate) color_texture: Texture2D,
    depth_stencil_target: DepthStencilTarget,
}

impl FBO {
    pub fn new(color_texture: Texture2D, depth_stencil_target: DepthStencilTarget) -> Self {
        let mut id = 0u32;
        gl_call!(gl::CreateFramebuffers(1, &mut id));
        // Bind color
        gl_call!(gl::NamedFramebufferTexture(id, gl::COLOR_ATTACHMENT0, color_texture.id, 0));
        // Bind depth & stencil
        match &depth_stencil_target {
            DepthStencilTarget::Texture2D(texture) => {
                gl_call!(gl::NamedFramebufferTexture(id, gl::DEPTH_STENCIL_ATTACHMENT, texture.id, 0));
            }
            DepthStencilTarget::RBO(rbo) => {
                gl_call!(gl::NamedFramebufferRenderbuffer(id, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo.id));
            }
        }
        if gl_call!(gl::CheckNamedFramebufferStatus(id, gl::FRAMEBUFFER)) != gl::FRAMEBUFFER_COMPLETE {
            panic!("Framebuffer {} is not complete", id);
        }
        FBO { id, color_texture, depth_stencil_target }
    }

    pub fn bind(&self) {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, self.id));
    }

    pub fn bind_default() {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));
    }
}

impl Drop for FBO {
    fn drop(&mut self) {
        gl_call!(gl::DeleteFramebuffers(1, &self.id));
    }
}