#[derive(Debug)]
pub struct UniformDefinition {
    id: gl::types::GLuint,
    name: String,
    data_size: gl::types::GLint,
}

impl UniformDefinition {
    pub fn new(id: gl::types::GLuint, name: String, data_size: gl::types::GLint) -> Self {
        Self {
            id: id,
            name: name,
            data_size: data_size,
        }
    }
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

impl Uniform {
    pub fn from_type(unif_type: gl::types::GLenum, def: UniformDefinition) -> Uniform {
        match unif_type {
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
        }
    }
}
