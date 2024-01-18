use crate::core::pipeline::mgl::attr::uniform::{Uniform, UniformDefinition};
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub enum ShaderIssue {
    CompileError(String),
    LinkError(String),
    StringConversionError(String),
}

impl std::fmt::Display for ShaderIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompileError(msg) => write!(f, "Shader compile error: {}", msg),
            Self::LinkError(msg) => write!(f, "Shader link issue: {}", msg),
            Self::StringConversionError(msg) => {
                write!(f, "Failed to convert between string represations: {}", msg)
            }
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

pub struct ShaderProgram {
    id: gl::types::GLuint,
    uniform_defs: Vec<Uniform>,
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

        const MAX_UNIFORM_NAME_LEN: usize = 512;
        let mut uniform_count: GLint = 0;
        let mut name_buf = [0i8; MAX_UNIFORM_NAME_LEN + 1];
        let mut name_len = 0;
        let mut unif_size = 0;
        let mut unif_type = 0;

        let mut unifs = Vec::<Uniform>::new();
        unifs.reserve(uniform_count as usize);

        unsafe {
            gl::GetProgramiv(program_id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
            for uid in 0u32..(uniform_count as u32) {
                gl::GetActiveUniform(
                    program_id,
                    uid,
                    MAX_UNIFORM_NAME_LEN as i32,
                    &mut name_len,
                    &mut unif_size,
                    &mut unif_type,
                    name_buf.as_mut_ptr(),
                );

                let unif_name = unsafe {
                    match CString::from(CStr::from_ptr(name_buf.as_ptr())).into_string() {
                        Ok(s) => s,
                        Err(err) => {
                            return Err(ShaderIssue::StringConversionError(format!(
                                "Uniform name: {}",
                                err.to_string()
                            )))
                        }
                    }
                };

                let def = UniformDefinition::new(uid, unif_name, unif_size);

                let unif = Uniform::from_type(unif_type, def);

                unifs.push(unif);
            }

            for unif in &unifs {
                println!("Uniform: {:?}", unif);
            }
        }

        return Ok(Self {
            id: program_id,
            uniform_defs: unifs,
        });
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
