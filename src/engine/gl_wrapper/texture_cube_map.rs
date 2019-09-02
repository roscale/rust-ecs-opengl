use image::GenericImageView;
use std::os::raw::c_void;
use std::path::Path;

#[derive(Debug)]
pub struct TextureCubeMap {
    id: u32,
}

impl TextureCubeMap {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenTextures(1, &mut id));
        TextureCubeMap { id }
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id));
        self
    }

    pub fn activate(unit: u32) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + unit));
    }

    pub fn fill(&self, filenames: &[&str; 6]) -> &Self {
        for i in 0..6 {
            let filename = filenames[i];

            let img = image::open(&filename);
            let img = match img {
                Ok(img) => img,
                Err(err) => panic!("Filename: {}, error: {}", filename, err.to_string())
            };

            let (width, height) = img.dimensions();
            let (width, height) = (width as i32, height as i32);

            let pixels = img.raw_pixels();

            gl_call!(gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0,
                                gl::RGB as i32, width, height, 0, gl::RGB,
                                gl::UNSIGNED_BYTE, pixels.as_ptr() as *const c_void));
        }

        gl_call!(gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32));

        self
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        gl_call!(gl::DeleteTextures(1, &self.id))
    }
}