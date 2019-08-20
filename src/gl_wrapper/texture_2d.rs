use image::GenericImageView;
use std::os::raw::c_void;

#[derive(Clone)]
pub struct Texture2D {
    pub id: u32
}

impl Texture2D {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::GenTextures(1, &mut id));
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
            Err(err) => panic!(err.to_string())
        };

        let (width, height) = img.dimensions();
        let img = img.flipv();
        let (width, height) = (width as i32, height as i32);

        let pixels = img.raw_pixels();

        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0,
                                gl::RGB as i32, width, height, 0, gl::RGB,
                                gl::UNSIGNED_BYTE, pixels.as_ptr() as *const c_void));

        gl_call!(gl::GenerateMipmap(gl::TEXTURE_2D));

        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));

        self
    }

    pub fn allocate_color(&self, width: i32, height: i32) {
        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0,
                                gl::RGB as i32, width, height, 0, gl::RGB,
                                gl::UNSIGNED_BYTE, 0 as *const c_void));

        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32));
    }

    pub fn allocate_depth_stencil(&self, width: i32, height: i32) {
        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0,
                                gl::DEPTH24_STENCIL8 as i32, width, height, 0, gl::DEPTH_STENCIL,
                                gl::UNSIGNED_INT_24_8, 0 as *const c_void));
    }
}

//impl Drop for Texture2D {
//    fn drop(&mut self) {
//        gl_call!(gl::DeleteTextures(1, &self.id))
//    }
//}