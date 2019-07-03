use gl;
use std;
use std::ffi::{CString, CStr};

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
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

pub struct Program {
    id: gl::types::GLuint
}

impl Program {
    pub fn use_program(&self) {
        gl_call!(gl::UseProgram(self.id));
    }

    pub fn from_shaders(vertex: Shader, fragment: Shader) -> Result<Program, String> {
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
        Ok(Program { id: program_id })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.id));
    }
}