use crate::gl_wrapper::fbo::{FBO, DepthStencilTarget};
use crate::gl_wrapper::texture_2d::Texture2D;
use crate::gl_wrapper::rbo::RBO;
use crate::containers::CONTAINER;
use crate::shaders::post_processing::{KernelShader, GaussianBlurShader};
use crate::shapes::PredefinedShapes;

pub trait PPEffect: Send + Sync {
    fn apply(&self, input: &FBO) -> &FBO;
    fn apply_to_screen(&self, input: &FBO);
}

pub struct Kernel {
    kernel: Vec<f32>,
    fb: FBO
}

impl Kernel {
    pub fn new(kernel: Vec<f32>) -> Self {
        // Validate kernel size
        let sq = f32::sqrt(kernel.len() as f32);
        if !(sq == sq.floor() && sq.floor() as u32 % 2 == 1) {
            panic!("Kernel len must be square of odd number")
        }

        let color_texture = Texture2D::new();
        color_texture.bind();
        color_texture.allocate_color(800, 800);

        let depth_stencil_rb = RBO::new();
        depth_stencil_rb.bind();
        depth_stencil_rb.create_depth_stencil(800, 800);

        Kernel {
            kernel,
            fb: FBO::new(color_texture, DepthStencilTarget::RBO(depth_stencil_rb))
        }
    }

    fn _apply(&self, input: &FBO) {
        gl_call!(gl::ClearColor(1.0, 0.5, 1.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT));

        let pp_shader = CONTAINER.get_local::<KernelShader>();
        let quad_vao = CONTAINER.get_local::<PredefinedShapes>().shapes.get("unit_quad").unwrap();

        pp_shader.bind_screen_texture(&input.color_texture);
        pp_shader.bind_kernel(&self.kernel);
        quad_vao.bind();
        gl_call!(gl::Disable(gl::DEPTH_TEST));
        gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, 6));
    }
}

impl PPEffect for Kernel {
    fn apply(&self, input: &FBO) -> &FBO {
        self.fb.bind();
        self._apply(input);
        &self.fb
    }

    fn apply_to_screen(&self, input: &FBO) {
        FBO::bind_default();
        self._apply(input);
    }
}

pub struct GaussianBlur {
    kernel: Vec<f32>,
    v_pass: FBO,
    h_pass: FBO
}

impl GaussianBlur {
    // TODO specify kernel size / sigma(intensity) and precalculate the kernel
    pub fn new(kernel: Vec<f32>) -> Self {
        // Validate kernel size
        if kernel.len() % 2 != 1 {
            panic!("GaussianBlur kernel len must be an odd number")
        }

        let create_fb = || {
            let color_texture = Texture2D::new();
            color_texture.bind();
            color_texture.allocate_color(800, 800);

            let depth_stencil_rb = RBO::new();
            depth_stencil_rb.bind();
            depth_stencil_rb.create_depth_stencil(800, 800);

            // TODO Prefer composition over setters
            FBO::new(color_texture, DepthStencilTarget::RBO(depth_stencil_rb))
        };

        GaussianBlur {
            kernel,
            v_pass: create_fb(),
            h_pass: create_fb(),
        }
    }

    fn _apply(&self, input: &FBO, to_screen: bool) {
        let pp_shader = CONTAINER.get_local::<GaussianBlurShader>();
        let quad_vao = CONTAINER.get_local::<PredefinedShapes>().shapes.get("unit_quad").unwrap();

        // v pass
        pp_shader.bind_screen_texture(&input.color_texture);
        pp_shader.bind_kernel(&self.kernel, true);
        quad_vao.bind();

        self.v_pass.bind();
        gl_call!(gl::ClearColor(1.0, 0.5, 1.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT));
        gl_call!(gl::Disable(gl::DEPTH_TEST));
        gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, 6));

        // h pass
        pp_shader.bind_screen_texture(&self.v_pass.color_texture);
        pp_shader.bind_kernel(&self.kernel, false);
        quad_vao.bind();

        if !to_screen { self.h_pass.bind(); } else { FBO::bind_default(); }
        gl_call!(gl::ClearColor(1.0, 0.5, 1.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT));
        gl_call!(gl::Disable(gl::DEPTH_TEST));
        gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, 6));
    }
}

impl PPEffect for GaussianBlur {
    fn apply(&self, input: &FBO) -> &FBO {
        self._apply(input, false);
        &self.h_pass
    }

    fn apply_to_screen(&self, input: &FBO) {
        self._apply(input, true);
    }
}