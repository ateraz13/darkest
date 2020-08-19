pub mod mesh3d;
pub use gl::types::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AttributeType {
    Vec2, Vec3, Vec4, Float
}

impl AttributeType {

    #[allow(dead_code)]
    pub fn count(&self) -> GLuint {
        match self {
            Self::Float => 1,
            Self::Vec2  => 2,
            Self::Vec3  => 3,
            Self::Vec4  => 4,
        }
    }

    #[allow(dead_code)]
    pub fn gl_type(&self) -> GLenum {
        match self {
            Self::Float => gl::FLOAT,
            Self::Vec2  => gl::FLOAT,
            Self::Vec3  => gl::FLOAT,
            Self::Vec4  => gl::FLOAT,
        }
    }
}
