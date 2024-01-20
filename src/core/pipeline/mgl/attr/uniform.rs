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

trait UniformUpload {
    type Item;
    fn upload(&self, item: Self::Item);
}

macro_rules! impl_from_uniform_def {
    (($struct:ident, $enum:ident)) => {
        #[derive(Debug)]
        pub struct $struct {
            def: UniformDefinition,
        }

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
    ($(($gl_type_rest:ident, $struct_rest:ident, $enum_rest:ident),)*) => {
        #[derive(Debug)]
        pub enum Uniform {
            $($enum_rest($struct_rest),)*
        }

        impl Uniform {
            pub fn from_type(gl_type: u32, def: UniformDefinition) -> Uniform {
                match gl_type {
                    $(gl::$gl_type_rest => Uniform::$enum_rest(def.into()),)*
                        _ => {panic!("Unimplemented uniform type!");}
                }
            }
        }

        $(impl_from_uniform_def!{($struct_rest, $enum_rest)})*
    }
}

impl_from_uniform_defs! {
    (INT, IntUniform, Int),
    (UNSIGNED_INT, UIntUniform, UInt),
    (BOOL, BoolUniform, Bool),
    (FLOAT, FloatUniform, Float),
    (DOUBLE, DoubleUniform, Double),
    (FLOAT_VEC2, Vec2Uniform, Vec2),
    (FLOAT_VEC3, Vec3Uniform, Vec3),
    (FLOAT_VEC4, Vec4Uniform, Vec4),
    (FLOAT_MAT2, Mat2Uniform, Mat2),
    (FLOAT_MAT3, Mat3Uniform, Mat3),
    (FLOAT_MAT4, Mat4Uniform, Mat4),
    (INT_VEC2, Vec2iUniform, Vec2i),
    (INT_VEC3, Vec3iUniform, Vec3i),
    (INT_VEC4, Vec4iUniform, Vec4i),
    (UNSIGNED_INT_VEC2, Vec2uiUniform, Vec2ui),
    (UNSIGNED_INT_VEC3, Vec3uiUniform, Vec3ui),
    (BOOL_VEC2, Vec2bUniform, Vec2b),
    (BOOL_VEC3, Vec3bUniform, Vec3b),
    (BOOL_VEC4, Vec4bUniform, Vec4b),
    (FLOAT_MAT2x3, Mat2x3Uniform, Mat2x3),
    (FLOAT_MAT2x4, Mat2x4Uniform, Mat2x4),
    (FLOAT_MAT3x2, Mat3x2Uniform, Mat3x2),
    (FLOAT_MAT3x4, Mat3x4Uniform, Mat3x4),
    (FLOAT_MAT4x2, Mat4x2Uniform, Mat4x2),
    (DOUBLE_MAT2, Mat2dUniform, Mat2d),
    (DOUBLE_MAT3, Mat3dUniform, Mat3d),
    (DOUBLE_MAT4, Mat4dUniform, Mat4d),
    (DOUBLE_MAT2x3, Mat2x3dUniform, Mat2x3d),
    (DOUBLE_MAT2x4, Mat2x4dUniform, Mat2x4d),
    (DOUBLE_MAT3x2, Mat3x2dUniform, Mat3x2d),
    (DOUBLE_MAT3x4, Mat3x4dUniform, Mat3x4d),
    (DOUBLE_MAT4x2, Mat4x2dUniform, Mat4x2d),
    (DOUBLE_MAT4x3, Mat4x3dUniform, Mat4x3d),
    (SAMPLER_1D, Sampler1DUniform, Sampler1D),
    (SAMPLER_2D, Sampler2DUniform, Sampler2D),
    (SAMPLER_3D, Sampler3DUniform, Sampler3D),
    (SAMPLER_CUBE, SamplerCubeUniform, SamplerCube),
    (SAMPLER_1D_SHADOW, Sampler1DShadowUniform, Sampler1DShadow),
    (SAMPLER_2D_SHADOW, Sampler2DShadowUniform, Sampler2DShadow),
    (SAMPLER_1D_ARRAY, Sampler1DArrayUniform, Sampler1DArray),
    (SAMPLER_2D_ARRAY, Sampler2DArrayUniform, Sampler2DArray),
    (SAMPLER_1D_ARRAY_SHADOW, Sampler1DArrayShadowUniform, Sampler1DArrayShadow),
    (SAMPLER_2D_ARRAY_SHADOW, Sampler2DArrayShadowUniform, Sampler2DArrayShadow),
    (SAMPLER_2D_MULTISAMPLE, Sampler2DMSUniform, Sampler2DMS),
    (SAMPLER_2D_MULTISAMPLE_ARRAY, Sampler2DMSArrayUniform, Sampler2DMSArray),
    (SAMPLER_CUBE_SHADOW, SamplerCubeShadowUniform, SamplerCubeShadow),
    (SAMPLER_BUFFER, SamplerBufferUniform, SamplerBuffer),
    (SAMPLER_2D_RECT, Sampler2DRectUniform, Sampler2DRect),
    (SAMPLER_2D_RECT_SHADOW, Sampler2DRectShadowUniform, Sampler2DRectShadow),
    (INT_SAMPLER_1D, Sampler1DiUniform, Sampler1Di),
    (INT_SAMPLER_2D, Sampler2DiUniform, Sampler2Di),
    (INT_SAMPLER_3D, Sampler3DiUniform, Sampler3Di),
    (INT_SAMPLER_CUBE, SamplerCubeiUniform, SamplerCubei),
    (INT_SAMPLER_1D_ARRAY, Sampler1DArrayiUniform, Sampler1DArrayi),
    (INT_SAMPLER_2D_ARRAY, Sampler2DArrayiUniform, Sampler2DArrayi),
    (INT_SAMPLER_2D_MULTISAMPLE, Sampler2DMSiUniform, Sampler2DMSi),
    (INT_SAMPLER_2D_MULTISAMPLE_ARRAY, Sampler2DMSArrayiUniform, Sampler2DMSArrayi),
    (INT_SAMPLER_BUFFER, SamplerBufferiUniform, SamplerBufferi),
    (INT_SAMPLER_2D_RECT, Sampler2DRectiUniform, Sampler2DRecti),
    (UNSIGNED_INT_SAMPLER_1D, Sampler1DuUniform, Sampler1Du),
    (UNSIGNED_INT_SAMPLER_2D, Sampler2DuUniform, Sampler2Du),
    (UNSIGNED_INT_SAMPLER_3D, Sampler3DuUniform, Sampler3Du),
    (UNSIGNED_INT_SAMPLER_CUBE, SamplerCubeuUniform, SamplerCubeu),
    (UNSIGNED_INT_SAMPLER_1D_ARRAY, Sampler1DArrayuUniform, Sampler1DArrayu),
    (UNSIGNED_INT_SAMPLER_2D_ARRAY, Sampler2DArrayuUniform, Sampler2DArrayu),
    (UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE, Sampler2DMSuUniform, Sampler2DMSu),
    (UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY, Sampler2DMSArrayuUniform, Sampler2DMSArrayu),
    (UNSIGNED_INT_SAMPLER_BUFFER, SamplerBufferuUniform, SamplerBufferu),
    (UNSIGNED_INT_SAMPLER_2D_RECT, Sampler2DuiUniform, Sampler2Dui),
    (IMAGE_1D, Image1DUniform, Image1D),
    (IMAGE_2D, Image2DUniform, Image2D),
    (IMAGE_3D, Image3DUniform, Image3D),
    (IMAGE_2D_RECT, Image2DRectUniform, Image2DRect),
    (IMAGE_CUBE, ImageCubeUniform, ImageCube),
    (IMAGE_BUFFER, ImageBufferUniform, ImageBuffer),
    (IMAGE_1D_ARRAY, Image1DArrayUniform, Image1DArray),
    (IMAGE_2D_ARRAY, Image2DArrayUniform, Image2DArray),
    (IMAGE_2D_MULTISAMPLE, Image2DMSUniform, Image2DMS),
    (IMAGE_2D_MULTISAMPLE_ARRAY, Image2DMSArrayUniform, Image2DMSArray),
    (INT_IMAGE_1D, Image1DiUniform, Image1Di),
    (INT_IMAGE_2D, Image2DiUniform, Image2Di),
    (INT_IMAGE_3D, Image3DiUniform, Image3Di),
    (INT_IMAGE_2D_RECT, Image2DRectiUniform, Image2DRecti),
    (INT_IMAGE_CUBE, ImageCubeiUniform, ImageCubei),
    (INT_IMAGE_BUFFER, ImageBufferiUniform, ImageBufferi),
    (INT_IMAGE_1D_ARRAY, Image1DArrayiUniform, Image1DArrayi),
    (INT_IMAGE_2D_ARRAY, Image2DArrayiUniform, Image2DArrayi),
    (INT_IMAGE_2D_MULTISAMPLE, Image2DMSiUniform, Image2DMSi),
    (INT_IMAGE_2D_MULTISAMPLE_ARRAY, Image2DMSArrayiUniform, Image2DMSArrayi),
    (UNSIGNED_INT_IMAGE_1D, Image1DuUniform, Image1Du),
    (UNSIGNED_INT_IMAGE_2D, Image2DuUniform, Image2Du),
    (UNSIGNED_INT_IMAGE_3D, Image3DuUniform, Image3Du),
    (UNSIGNED_INT_IMAGE_2D_RECT, Image2DRectuUniform, Image2DRectu),
    (UNSIGNED_INT_IMAGE_CUBE, ImageCubeuUniform, ImageCubeu),
    (UNSIGNED_INT_IMAGE_BUFFER, ImageBufferuUniform, ImageBufferu),
    (UNSIGNED_INT_IMAGE_1D_ARRAY, Image1DArrayuUniform, Image1DArrayu),
    (UNSIGNED_INT_IMAGE_2D_ARRAY, Image2DArrayuUniform, Image2DArrayu),
    (UNSIGNED_INT_IMAGE_2D_MULTISAMPLE, Image2DMSuUniform, Image2DMSu),
    (UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY, Image2DMSArrayuUniform, Image2DMSArrayu),
    (UNSIGNED_INT_ATOMIC_COUNTER, AtomicCounteruiUniform, AtomicCounterui),
}
