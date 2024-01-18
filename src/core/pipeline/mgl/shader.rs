use std::ffi::{CStr, CString};

#[derive(Debug)]
pub enum ShaderIssue {
    CompileError(String),
    LinkError(String),
}

impl std::fmt::Display for ShaderIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompileError(msg) => write!(f, "Shader compile error: {}", msg),
            Self::LinkError(msg) => write!(f, "Shader link issue: {}", msg),
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

pub struct ShaderProgram {
    id: gl::types::GLuint,
}

fn prepare_whitespaced_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

impl ShaderProgram {
    pub fn set_active(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, ShaderIssue> {
        use gl::types::*;

        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()) };
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut mesg_len: GLint = 1;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut mesg_len);
            }

            let mesg = prepare_whitespaced_cstring_with_len(mesg_len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    mesg_len,
                    std::ptr::null_mut(),
                    mesg.as_ptr() as *mut GLchar,
                )
            }

            return Err(ShaderIssue::LinkError(mesg.to_string_lossy().into_owned()));
        }

        // FIXME: Retreve all the defined uniforms in the shader program.

        return Ok(Self { id: program_id });
    }
}

impl Shader {
    pub fn from_source(
        shader_source: &CStr,
        shader_type: gl::types::GLenum,
    ) -> Result<Self, ShaderIssue> {
        use gl::types::*;

        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1, &shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 1;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mesg = prepare_whitespaced_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), mesg.as_ptr() as *mut GLchar);
            }

            return Err(ShaderIssue::CompileError(
                mesg.to_string_lossy().into_owned(),
            ));
        }

        return Ok(Self { id });
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
