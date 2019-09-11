use gl;
use std;
use std::ffi::{CString, CStr};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ShaderPart {
    id: u32,
}

impl ShaderPart {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<ShaderPart, String> {
        let id = shader_from_source(source, kind)?;
        Ok(ShaderPart { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<ShaderPart, String> {
        ShaderPart::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<ShaderPart, String> {
        ShaderPart::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for ShaderPart {
    fn drop(&mut self) {
        gl_call!(gl::DeleteShader(self.id));
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = gl_call!(gl::CreateShader(kind));
    gl_call!(gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null()));
    gl_call!(gl::CompileShader(id));

    let mut success: gl::types::GLint = 1;
    gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success));

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        gl_call!(gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len));

        let error = create_whitespace_cstring_with_len(len as usize);

        gl_call!(gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        ));

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ShaderProgram {
    id: u32,
    uniform_cache: RefCell<HashMap<String, i32>>
}

impl ShaderProgram {
    pub fn use_program(&self) {
        gl_call!(gl::UseProgram(self.id));
    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        let location = self.uniform_cache.borrow().get(name).cloned();
        match location {
            None => {
                let c_name = CString::new(name).unwrap();
                let location = gl_call!(gl::GetUniformLocation(self.id, c_name.as_ptr()));
                // Error checking
                if location == -1 {
                    panic!("Can't find uniform '{}' in program with id: {}", name, self.id);
                }
                println!("New uniform location {}: {}", &name, &location);
                self.uniform_cache.borrow_mut().insert(name.to_owned(), location);
                location
            },
            Some(location) => location,
        }
    }

    pub fn set_uniform3f(&self, name: &str, values: &[f32]) -> &Self {
        let location = self.get_uniform_location(name);
        gl_call!(gl::Uniform3f(location, values[0], values[1], values[2]));
        self
    }

    pub fn set_uniform4f(&self, name: &str, values: &[f32]) -> &Self {
        let location = self.get_uniform_location(name);
        gl_call!(gl::Uniform4f(location, values[0], values[1], values[2], values[3]));
        self
    }

    pub fn set_uniform_matrix4fv(&self, name: &str, matrix: *const f32) -> &Self {
        let location = self.get_uniform_location(name);
        gl_call!(gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix));
        self
    }

    pub fn set_uniform1fv(&self, name: &str, vec: &[f32]) -> &Self {
        let location = self.get_uniform_location(name);
        gl_call!(gl::Uniform1fv(location, vec.len() as i32, vec.as_ptr()));
        self
    }

    pub fn set_uniform1f(&self, name: &str, value: f32) -> &Self {
        let location = self.get_uniform_location(name);
        gl_call!(gl::Uniform1f(location, value));
        self
    }

    pub fn set_uniform1i(&self, name: &str, value: i32) -> &Self {
        let location = self.get_uniform_location(name);
        gl_call!(gl::Uniform1i(location, value));
        self
    }

    pub fn from_shaders(vertex: ShaderPart, fragment: ShaderPart) -> Result<ShaderProgram, String> {
        let program_id = gl_call!(gl::CreateProgram());

        gl_call!(gl::AttachShader(program_id, vertex.id));
        gl_call!(gl::AttachShader(program_id, fragment.id));
        gl_call!(gl::LinkProgram(program_id));

        // Error checking
        let mut success: gl::types::GLint = 1;
        gl_call!(gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success));

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl_call!(gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len));

            let error = create_whitespace_cstring_with_len(len as usize);

            gl_call!(gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            ));

            return Err(error.to_string_lossy().into_owned());
        }

        gl_call!(gl::DetachShader(program_id, vertex.id));
        gl_call!(gl::DetachShader(program_id, fragment.id));
        Ok(ShaderProgram { id: program_id, uniform_cache: RefCell::new(HashMap::new()) })
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.id));
    }
}