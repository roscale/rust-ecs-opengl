use image::{GenericImageView, DynamicImage};
use std::os::raw::c_void;
use std::path::Path;

#[derive(Debug)]
pub enum TextureFormat {
    Unknown,
    RGB,
    RGBA,
    DepthStencil
}

impl TextureFormat {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            TextureFormat::RGB => gl::RGB,
            TextureFormat::RGBA => gl::RGBA,
            TextureFormat::DepthStencil => panic!("Use the to_gl_enum_sized() function instead"),
            _ => panic!("Uninitialized texture"),
        }
    }

    pub fn to_gl_enum_sized(&self) -> u32 {
        match self {
            TextureFormat::RGB => gl::RGB8,
            TextureFormat::RGBA => gl::RGBA8,
            TextureFormat::DepthStencil => gl::DEPTH24_STENCIL8,
            _ => panic!("Uninitialized texture"),
        }
    }
}

/*
let img = image::open(filename);
let img = match img {
    Ok(img) => img,
    Err(err) => panic!("Filename: {}, error: {}", filename, err.to_string())
};

let (width, height) = img.dimensions();
let img = img.flipv();
let (width, height) = (width as i32, height as i32);
*/

#[derive(Debug)]
pub struct Texture2D {
    pub(crate) id: u32,
    format: TextureFormat,
    width: u32,
    height: u32,
    mipmap_levels: u32,
}

impl Texture2D {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        gl_call!(gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id));
        gl_call!(gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32));
        gl_call!(gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));
        Texture2D { id, format: TextureFormat::Unknown, width: 0, height: 0, mipmap_levels: 0 }
    }

    pub fn allocate(&mut self, format: TextureFormat, width: u32, height: u32, mipmap_levels: u32) {
        gl_call!(gl::TextureStorage2D(
            self.id, mipmap_levels as i32,
            format.to_gl_enum_sized(),
            width as i32, height as i32));
        self.format = format;
        self.width = width;
        self.height = height;
        self.mipmap_levels = mipmap_levels;
    }

    pub fn update(&mut self, xoffset: u32, yoffset: u32, img: &DynamicImage) {
        gl_call!(gl::TextureSubImage2D(
            self.id, 0,
            xoffset as i32, yoffset as i32, img.width() as i32, img.height() as i32,
            self.format.to_gl_enum(), gl::UNSIGNED_BYTE,
            img.raw_pixels().as_ptr() as *mut c_void));

        gl_call!(gl::GenerateTextureMipmap(self.id));
    }

    pub fn activate(&self, unit: u32) -> &Self {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + unit));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.id));
        self
    }

    pub fn unbind(&self) -> &Self {
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
        self
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        gl_call!(gl::DeleteTextures(1, &self.id))
    }
}