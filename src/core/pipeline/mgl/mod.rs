pub mod s3tc;

// MODULE: Move GL
// This modules defines structures which can are used
// for transforming data stored on the CPU into their OpenGL represantations

pub mod attr {

    pub mod mesh3d {

        use std::convert::TryInto;

        use gl::types::*;

        #[derive(Debug)]
        pub enum AttributeType {
            Vec2, Vec3, Vec4, Float
        }

        impl AttributeType {
            pub fn count(&self) -> GLuint {
                match self {
                    Float => 1,
                    Vec2  => 2,
                    Vec3  => 3,
                    Vec4  => 4,
                }
            }

            pub fn gl_type(&self) -> GLenum {
                match self {
                    Float => gl::FLOAT,
                    Vec2  => gl::FLOAT,
                    Vec3  => gl::FLOAT,
                    Vec4  => gl::FLOAT,
                }
            }
        }

        #[derive(Debug)]
        pub struct VertexAttributes {
            pub pos_comp_type: AttributeType,
            // NOTE: we do not use vector types for attributes because we may want
            // different number of components for some attributes
            pub positions: Vec<f32>, // 2 or 3 components per position
            pub normals: Vec<f32>, // 3 components per normal
            pub uvs: Vec<f32> // 2 components per uv
        }

        pub use crate::s3tc::Image;

        pub struct LightMaps {
            pub diffuse: Image,
            pub specular: Image,
        }

        // If you change this prepare to change GL code
        type MeshIndex = GLushort;

        #[derive(Debug)]
        pub struct IndexedMesh {
            pub indices: Vec<MeshIndex>,
            pub attributes: VertexAttributes,
        }

        impl IndexedMesh {

            // What if indices exeed the number of positions ?
            // Probably don't need to worry but also be careful !
            pub fn new(indices: Vec<MeshIndex>, attrs: VertexAttributes) -> Self {
                Self {
                    indices: indices,
                    attributes: attrs,
                }
            }

            pub fn vertex_count (&self) -> GLuint {
                self.indices.len() as GLuint
            }

            pub fn index_buffer_size(&self) -> GLsizeiptr {
                (self.indices.len() * std::mem::size_of::<MeshIndex>()).try_into().unwrap()
            }

            pub fn index_buffer_ptr(&self) -> *const GLvoid {
                self.indices.as_ptr() as *const GLvoid
            }

            // pub fn index_buffer_size(&self) -> GLuint {
            //     self.indices.len() * std::mem::size_of::<MeshIndex>()
            // }
        }


        impl VertexAttributes {

            pub fn position_buffer_size(&self) -> GLsizeiptr {
                (self.positions.len() * std::mem::size_of::<f32>()) as GLsizeiptr
            }

            pub fn normal_buffer_size(&self) -> GLsizeiptr {
                (self.normals.len() * std::mem::size_of::<f32>()) as GLsizeiptr
            }

            pub fn uv_buffer_size(&self) -> GLsizeiptr {
                (self.uvs.len() * std::mem::size_of::<f32>()) as GLsizeiptr
            }

            pub unsafe fn position_buffer_ptr(&self) -> *const GLvoid {
                self.positions.as_ptr() as *const GLvoid
            }

            pub unsafe fn normal_buffer_ptr(&self) -> *const GLvoid {
                self.normals.as_ptr() as *const GLvoid
            }

            pub unsafe fn uv_buffer_ptr(&self) -> *const GLvoid {
                self.uvs.as_ptr() as *const GLvoid
            }
        }

    }
}

pub mod shaders {

    use std::ffi::{CString, CStr};

    #[derive(Debug)]
    pub enum ShaderIssue {
        CompileError(String),
        LinkError(String),
    }

    pub struct Shader {
        id: gl::types::GLuint,
    }

    pub struct ShaderProgram {
        id: gl::types::GLuint
    }

    fn prepare_whitespaced_cstring_with_len(len: usize) -> CString {

        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        buffer.extend([b' '].iter().cycle().take(len));
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    impl ShaderProgram {

        pub fn set_active(&self) {
            unsafe { gl::UseProgram(self.id); }
        }

        pub fn from_shaders(
            shaders: &[Shader]
        ) -> Result<Self, ShaderIssue> {

            use gl::types::*;

            let program_id = unsafe { gl::CreateProgram() };

            for shader in shaders {
                unsafe { gl::AttachShader(program_id, shader.id()); }
            }

            unsafe { gl::LinkProgram(program_id); }

            for shader in shaders {
                unsafe { gl::DetachShader(program_id, shader.id()) };
            }

            let mut success : GLint = 1;
            unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success); }

            if success == 0 {

                let mut mesg_len : GLint = 1;
                unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut mesg_len); }

                let mesg = prepare_whitespaced_cstring_with_len(mesg_len as usize);

                unsafe {
                    gl::GetProgramInfoLog(
                        program_id,
                        mesg_len,
                        std::ptr::null_mut(),
                        mesg.as_ptr() as *mut GLchar
                    )
                }

                return Err(ShaderIssue::LinkError(mesg.to_string_lossy().into_owned()));
            }

            return Ok(Self { id: program_id });
        }

    }

    impl Shader {

        pub fn from_source (
            shader_source: &CStr,
            shader_type: gl::types::GLenum
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
                    gl::GetShaderInfoLog(
                        id, len,
                        std::ptr::null_mut(),
                        mesg.as_ptr() as *mut GLchar
                    );
                }

                return Err(ShaderIssue::CompileError(mesg.to_string_lossy().into_owned()));
            }

            return Ok(Self { id });
        }

        pub fn id(&self) -> gl::types::GLuint {
            self.id
        }

    }

    impl Drop for Shader {
        fn drop (&mut self) {
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

}
