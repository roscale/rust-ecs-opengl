use image::GenericImageView;
use std::os::raw::c_void;
use std::path::Path;

#[derive(Debug)]
pub struct Texture2D {
    pub(crate) id: u32,
}

impl Texture2D {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id));
        Texture2D { id }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.id));
        self
    }

    pub fn activate(unit: u32) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + unit));
    }

    pub fn fill(&self, filename: &str) -> &Self {
        let img = image::open(filename);
        let img = match img {
            Ok(img) => img,
            Err(err) => panic!("Filename: {}, error: {}", filename, err.to_string())
        };

        let (width, height) = img.dimensions();
        let img = img.flipv();
        let (width, height) = (width as i32, height as i32);

        let pixels = img.raw_pixels();

        let mipmap_levels = 8;
        gl_call!(gl::TextureStorage2D(self.id, mipmap_levels, gl::RGB8, width, height));
        gl_call!(gl::TextureSubImage2D(
            self.id, 0,
            0, 0, width, height,
            gl::RGB, gl::UNSIGNED_BYTE,
            pixels.as_ptr() as *const c_void));

        gl_call!(gl::GenerateTextureMipmap(self.id));

        gl_call!(gl::TextureParameteri(self.id, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32));
        gl_call!(gl::TextureParameteri(self.id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));

        self
    }

    pub fn allocate_color(&self, width: i32, height: i32) {
        gl_call!(gl::TextureStorage2D(self.id, 1, gl::RGB8, width, height));

        gl_call!(gl::TextureParameteri(self.id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TextureParameteri(self.id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TextureParameteri(self.id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32));
        gl_call!(gl::TextureParameteri(self.id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32));
    }

    pub fn allocate_depth_stencil(&self, width: i32, height: i32) {
        gl_call!(gl::TextureStorage2D(self.id, 1, gl::DEPTH24_STENCIL8, width, height));
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        gl_call!(gl::DeleteTextures(1, &self.id))
    }
}