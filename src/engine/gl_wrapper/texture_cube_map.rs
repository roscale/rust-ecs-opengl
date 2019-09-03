use image::GenericImageView;
use std::os::raw::c_void;

#[derive(Debug)]
pub struct TextureCubeMap {
    id: u32,
}

impl TextureCubeMap {
    pub fn new(filenames: &[&str; 6]) -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::CreateTextures(gl::TEXTURE_CUBE_MAP_ARRAY, 1, &mut id));

        let (width, height) = {
            let dims = match image::open(&filenames[0]) {
                Ok(img) => img.dimensions(),
                Err(err) => panic!("Filename: {}, error: {}", filenames[0], err.to_string())
            };
            (dims.0 as i32, dims.1 as i32)
        };

        gl_call!(gl::TextureStorage3D(
            id, 1,
            gl::RGB8, width, height, 6));

        for i in 0..6 {
            let filename = filenames[i];

            let pixels = match image::open(&filename) {
                Ok(img) => img.raw_pixels(),
                Err(err) => panic!("Filename: {}, error: {}", filename, err.to_string())
            };

            gl_call!(gl::TextureSubImage3D(
                id, 0,
                0, 0, i as i32,
                width, height, 1,
                gl::RGB, gl::UNSIGNED_BYTE, pixels.as_ptr() as *const c_void));
        };

        gl_call!(gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32));
        gl_call!(gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32));
        gl_call!(gl::TextureParameteri(id, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32));

        TextureCubeMap { id }
    }

    pub fn activate(unit: u32) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + unit));
    }

    pub fn bind(&self) -> &Self {
        gl_call!(gl::BindTexture(gl::TEXTURE_CUBE_MAP_ARRAY, self.id));
        self
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        gl_call!(gl::DeleteTextures(1, &self.id))
    }
}