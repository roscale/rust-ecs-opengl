use crate::gl_wrapper::fbo::FBO;
use crate::gl_wrapper::texture_2d::Texture2D;
use crate::gl_wrapper::rbo::RBO;
use crate::containers::global_instances::CONTAINER;
use crate::shaders::post_processing::PostProcessingShader;
use crate::gl_wrapper::vao::VAO;

pub trait PPEffect: Send + Sync {
    fn apply(&self, input: &FBO) -> &FBO;
    fn apply_to_display(&self, input: &FBO);
}

pub struct Kernel3x3 {
    kernel: Vec<f32>,
    fb: FBO
}

impl Kernel3x3 {
    pub fn new(kernel: Vec<f32>) -> Self {
        let color_texture = Texture2D::new();
        color_texture.bind();
        color_texture.allocate_color(800, 800);

        let depth_stencil_rb = RBO::new();
        depth_stencil_rb.bind();
        depth_stencil_rb.create_depth_stencil(800, 800);

        // TODO Prefer composition over setters
        let mut fb = FBO::new();
        fb.bind();
        fb.attach_color_texture(&color_texture);
        fb.attach_depth_stencil_renderbuffer(&depth_stencil_rb);

        Kernel3x3 {
            kernel,
            fb
        }
    }

    fn _apply(&self, input: &FBO) {
        gl_call!(gl::Viewport(0, 0, 800, 800));
        gl_call!(gl::ClearColor(1.0, 0.5, 1.0, 1.0));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT));

        let pp_shader = CONTAINER.get_local::<PostProcessingShader>();
        let quad_vao = CONTAINER.get_local::<VAO>();

        pp_shader.bind_screen_texture(input.color_texture.as_ref().unwrap());
        pp_shader.bind_kernel(&self.kernel);
        quad_vao.bind();
        gl_call!(gl::Disable(gl::DEPTH_TEST));
        gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, 6));
    }
}

impl PPEffect for Kernel3x3 {
    fn apply(&self, input: &FBO) -> &FBO {
        self.fb.bind();
        self._apply(input);
        &self.fb
    }

    fn apply_to_display(&self, input: &FBO) {
        FBO::bind_default();
        self._apply(input);
    }
}