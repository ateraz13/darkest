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

#[derive(Debug)]
pub struct UniformDefinition {
    id: gl::types::GLuint,
    name: String,
    data_size: gl::types::GLint,
}

#[derive(Debug)]
pub struct IntUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct ShortUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct LongUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct UIntUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct UShortUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct ULongUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct FloatUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct HalfUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct DoubleUniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct Vec2Uniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct Vec3Uniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct Vec4Uniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct Mat2Uniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct Mat3Uniform {
    def: UniformDefinition,
}
#[derive(Debug)]
pub struct Mat4Uniform {
    def: UniformDefinition,
}

macro_rules! impl_from_uniform_def {
    ($struct:ident => $enum:ident) => {
        impl From<$struct> for Uniform {
            fn from(other: $struct) -> Uniform {
                Uniform::$enum(other)
            }
        }

        impl From<UniformDefinition> for $struct {
            fn from(def: UniformDefinition) -> $struct {
                $struct { def: def }
            }
        }
    };
}

macro_rules! impl_from_uniform_defs {
    ($struct:ident => $enum:ident$(,$struct_rest:ident => $enum_rest:ident)*) => {
        impl_from_uniform_def!{$struct => $enum}
        $(impl_from_uniform_def!{$struct_rest => $enum_rest})*
    }
}

impl_from_uniform_defs! {
    IntUniform => Int,
    ShortUniform => Short,
    LongUniform => Long,
    UIntUniform => UInt,
    UShortUniform => UShort,
    ULongUniform => ULong,
    FloatUniform => Float,
    HalfUniform => Half,
    DoubleUniform => Double,
    Vec2Uniform => Vec2,
    Vec3Uniform => Vec3,
    Vec4Uniform => Vec4,
    Mat2Uniform => Mat2,
    Mat3Uniform => Mat3,
    Mat4Uniform => Mat4
}

#[derive(Debug)]
pub enum Uniform {
    Int(IntUniform),
    Short(ShortUniform),
    Long(LongUniform),
    UInt(UIntUniform),
    UShort(UShortUniform),
    ULong(ULongUniform),
    Float(FloatUniform),
    Half(HalfUniform),
    Double(DoubleUniform),
    Vec2(Vec2Uniform),
    Vec3(Vec3Uniform),
    Vec4(Vec4Uniform),
    Mat2(Mat2Uniform),
    Mat3(Mat3Uniform),
    Mat4(Mat4Uniform),
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

        const MAX_UNIFORM_NAME_LEN: usize = 256;
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

                let def = UniformDefinition {
                    name: unif_name,
                    id: uid,
                    data_size: unif_size,
                };

                let unif = match unif_type {
                    GL_FLOAT => Uniform::Float(def.into()),
                    GL_FLOAT_VEC2 => Uniform::Vec2(def.into()),
                    GL_FLOAT_VEC3 => Uniform::Vec3(def.into()),
                    GL_FLOAT_VEC4 => Uniform::Vec4(def.into()),
                    GL_DOUBLE => Uniform::Double(def.into()),
                    // GL_DOUBLE_VEC2 => Uniform::DoubleVec2(def.into()),
                    // GL_DOUBLE_VEC3 => Uniform::DoubleVec3(def.into()),
                    // GL_DOUBLE_VEC4 => Uniform::DoubleVec4(def.into()),
                    GL_INT => Uniform::Int(def.into()),
                    // GL_INT_VEC2 => Uniform::Vec2i(def.into()),
                    // GL_INT_VEC3 => Uniform::Vec3i(def.into()),
                    // GL_INT_VEC4 => Uniform::Vec4i(def.into()),
                    GL_UNSIGNED_INT => Uniform::UInt(def.into()),
                    // GL_UNSIGNED_INT_VEC2 => Uniform::Vec2ui(def.into()),
                    // GL_UNSIGNED_INT_VEC3 => Uniform::Vec3ui(def.into()),
                    // GL_UNSIGNED_INT_VEC4 => Uniform::Vec4ui(def.into()),
                    // GL_BOOL => Uniform::Bool(def.into()),
                    // GL_BOOL_VEC2 => Uniform::Vec2b(def.into()),
                    // GL_BOOL_VEC3 => Uniform::Vec3b(def.into()),
                    // GL_BOOL_VEC4 => Uniform::Vec4(def.into()),
                    GL_FLOAT_MAT2 => Uniform::Mat2(def.into()),
                    GL_FLOAT_MAT3 => Uniform::Mat3(def.into()),
                    GL_FLOAT_MAT4 => Uniform::Mat4(def.into()),
                    // GL_FLOAT_MAT2x3 => Uniform::Mat2x3(def.into()),
                    // GL_FLOAT_MAT2x4 => Uniform::Mat2x4(def.into()),
                    // GL_FLOAT_MAT3x2 => Uniform::Mat3x2(def.into()),
                    // GL_FLOAT_MAT3x4 => Uniform::Mat3x4(def.into()),
                    // GL_FLOAT_MAT4x2 => Uniform::Mat4x2(def.into()),
                    // GL_FLOAT_MAT4x3 => Uniform::Mat4x3(def.into()),
                    // GL_DOUBLE_MAT2 => Uniform::Mat2d(def.into()),
                    // GL_DOUBLE_MAT3 => Uniform::Mat3d(def.into()),
                    // GL_DOUBLE_MAT4 => Uniform::Mat4d(def.into()),
                    // GL_DOUBLE_MAT2x3 => Uniform::Mat2x3d(def.into()),
                    // GL_DOUBLE_MAT2x4 => Uniform::Mat2x4d(def.into()),
                    // GL_DOUBLE_MAT3x2 => Uniform::Mat3x2d(def.into()),
                    // GL_DOUBLE_MAT3x4 => Uniform::Mat3x4d(def.into()),
                    // GL_DOUBLE_MAT4x2 => Uniform::Mat4x2d(def.into()),
                    // GL_DOUBLE_MAT4x3 => Uniform::Mat4x3d(def.into()),
                    // GL_SAMPLER_1D => Uniform::Sampler1D(def.into()),
                    // GL_SAMPLER_2D => Uniform::Sampler2D(def.into()),
                    // GL_SAMPLER_3D => Uniform::Sampler3D(def.into()),
                    // GL_SAMPLER_CUBE => Uniform::SamplerCube(def.into()),
                    // GL_SAMPLER_1D_SHADOW => Uniform::Sampler1DShadow(def.into()),
                    // GL_SAMPLER_2D_SHADOW => Uniform::Sampler2DShadow(def.into()),
                    // GL_SAMPLER_1D_ARRAY => Uniform::Sampler1DArray(def.into()),
                    // GL_SAMPLER_2D_ARRAY => Uniform::Sampler2DArray(def.into()),
                    // GL_SAMPLER_1D_ARRAY_SHADOW => Uniform::Sampler1DArrayShadow(def.into()),
                    // GL_SAMPLER_2D_ARRAY_SHADOW => Uniform::Sampler2DArrayShadow(def.into()),
                    // GL_SAMPLER_2D_MULTISAMPLE => Uniform::Sampler2DMS(def.into()),
                    // GL_SAMPLER_2D_MULTISAMPLE_ARRAY => Uniform::Sampler2DMSArray(def.into()),
                    // GL_SAMPLER_CUBE_SHADOW => Uniform::SamplerCubeShadow(def.into()),
                    // GL_SAMPLER_BUFFER => Uniform::SamplerBuffer(def.into()),
                    // GL_SAMPLER_2D_RECT => Uniform::Sampler2DRect(def.into()),
                    // GL_SAMPLER_2D_RECT_SHADOW => Uniform::Sampler2DRectShadow(def.into()),
                    // GL_INT_SAMPLER_1D => Uniform::Sampler1Di(def.into()),
                    // GL_INT_SAMPLER_2D => Uniform::Sampler2Di(def.into()),
                    // GL_INT_SAMPLER_3D => Uniform::Sampler3Di(def.into()),
                    // GL_INT_SAMPLER_CUBE => Uniform::SamplerCubei(def.into()),
                    // GL_INT_SAMPLER_1D_ARRAY => Uniform::Sampler1DArrayi(def.into()),
                    // GL_INT_SAMPLER_2D_ARRAY => Uniform::Sampler2DArrayi(def.into()),
                    // GL_INT_SAMPLER_2D_MULTISAMPLE => Uniform::Sampler2DMSi(def.into()),
                    // GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => Uniform::Sampler2DMSArrayi(def.into()),
                    // GL_INT_SAMPLER_BUFFER => Uniform::SamplerBufferi(def.into()),
                    // GL_INT_SAMPLER_2D_RECT => Uniform::Sampler2DRecti(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_1D => Uniform::Sampler1Du(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_2D => Uniform::Sampler2Du(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_3D => Uniform::Sampler3Du(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_CUBE => Uniform::SamplerCubeu(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_1D_ARRAY => Uniform::Sampler2DArrayu(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_2D_ARRAY => Uniform::Sampler2DArrayu(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE => Uniform::Sampler2DMSu(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => {
                    //     Uniform::Sampler2DMSArrayu(def.into())
                    // }
                    // GL_UNSIGNED_INT_SAMPLER_BUFFER => Uniform::SamplerBufferu(def.into()),
                    // GL_UNSIGNED_INT_SAMPLER_2D_RECT => Uniform::Sampler2Dui(def.into()),
                    // GL_IMAGE_1D => Uniform::Image1D(def.into()),
                    // GL_IMAGE_2D => Uniform::Image2D(def.into()),
                    // GL_IMAGE_3D => Uniform::Image3D(def.into()),
                    // GL_IMAGE_2D_RECT => Uniform::Image2DRect(def.into()),
                    // GL_IMAGE_CUBE => Uniform::ImageCube(def.into()),
                    // GL_IMAGE_BUFFER => Uniform::ImageBuffer(def.into()),
                    // GL_IMAGE_1D_ARRAY => Uniform::Image1DArray(def.into()),
                    // GL_IMAGE_2D_ARRAY => Uniform::Image2DArray(def.into()),
                    // GL_IMAGE_2D_MULTISAMPLE => Uniform::Image2DMS(def.into()),
                    // GL_IMAGE_2D_MULTISAMPLE_ARRAY => Uniform::Image2DMSArray(def.into()),
                    // GL_INT_IMAGE_1D => Uniform::Image1Di(def.into()),
                    // GL_INT_IMAGE_2D => Uniform::Image2Di(def.into()),
                    // GL_INT_IMAGE_3D => Uniform::Image3Di(def.into()),
                    // GL_INT_IMAGE_2D_RECT => Uniform::Image2DRecti(def.into()),
                    // GL_INT_IMAGE_CUBE => Uniform::ImageCubei(def.into()),
                    // GL_INT_IMAGE_BUFFER => Uniform::ImageBufferi(def.into()),
                    // GL_INT_IMAGE_1D_ARRAY => Uniform::Image1DArrayi(def.into()),
                    // GL_INT_IMAGE_2D_ARRAY => Uniform::Image2DArrayi(def.into()),
                    // GL_INT_IMAGE_2D_MULTISAMPLE => Uniform::Image2DMSi(def.into()),
                    // GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY => Uniform::Image2DMSArrayi(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_1D => Uniform::Image1Du(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_2D => Uniform::Image2Du(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_3D => Uniform::Image3Du(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_2D_RECT => Uniform::Image2DRectu(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_CUBE => Uniform::ImageCubeu(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_BUFFER => Uniform::ImageBufferu(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_1D_ARRAY => Uniform::Image1DArrayu(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_2D_ARRAY => Uniform::Image2DArrayu(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE => Uniform::Image2DMSu(def.into()),
                    // GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY => {
                    //     Uniform::Image2DMSArrayu(def.into())
                    // }
                    // GL_UNSIGNED_INT_ATOMIC_COUNTER => Uniform::AtomicCounterui(def.into()),
                };

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
