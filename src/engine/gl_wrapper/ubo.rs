use std::os::raw::c_void;

#[derive(PartialEq)]
pub enum GlslTypes {
    Float,
    Int,
    Vec2,
    Vec3,
    Vec4,
    Mat4
}

pub trait ComputeStd140LayoutSize {
    fn compute_std140_layout_size(&self) -> u32;
}

impl ComputeStd140LayoutSize for GlslTypes {
    fn compute_std140_layout_size(&self) -> u32 {
        let n = 4;
        match &self {
            GlslTypes::Float => n,
            GlslTypes::Int => n,
            GlslTypes::Vec2 => 2 * n,
            GlslTypes::Vec3 => 4 * n,
            GlslTypes::Vec4 => 4 * n,
            GlslTypes::Mat4 => 4 * (4 * n),
        }
    }
}

impl ComputeStd140LayoutSize for &[GlslTypes] {
    fn compute_std140_layout_size(&self) -> u32 {
        self.iter().fold(0, |acc, x| acc + x.compute_std140_layout_size())
    }
}

pub trait Std140 {
    fn get_std140_layout(&self) -> &'static [GlslTypes];
    fn write_to_ubo(&self, ubo: &UBO);
}

pub enum BufferUpdateFrequency {
    Never,
    Occasionally,
    Often,
}

pub struct UBO {
    pub(crate) id: u32,
    pub(crate) layout: &'static [GlslTypes]
}

impl UBO {
    pub fn new(layout: &'static [GlslTypes], update_frequency: BufferUpdateFrequency) -> Self {
        let update_frequency = match update_frequency {
            BufferUpdateFrequency::Never => gl::STATIC_DRAW,
            BufferUpdateFrequency::Occasionally => gl::DYNAMIC_DRAW,
            BufferUpdateFrequency::Often => gl::STREAM_DRAW,
        };

        let mut id: u32 = 0;
        gl_call!(gl::CreateBuffers(1, &mut id));
        gl_call!(gl::NamedBufferData(id, layout.compute_std140_layout_size() as isize, 0 as *const c_void, update_frequency));
        UBO { id, layout }
    }

    pub fn update<T: Std140>(&self, data: &T) {
        let compatible = self.layout.iter().eq(data.get_std140_layout().iter());
        if compatible {
            data.write_to_ubo(self);
        } else {
            panic!("Struct not compatible with UBO");
        }
    }

    pub fn bind(&self, binding_point: u32) -> &Self {
        gl_call!(gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, self.id));
        self
    }
}

impl Drop for UBO {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &self.id))
    }
}
