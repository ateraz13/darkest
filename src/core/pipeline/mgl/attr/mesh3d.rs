use gl::types::*;
use std::convert::TryInto;
use super::AttributeType;

type MeshIndex = GLuint;

#[derive(Debug)]
pub struct VertexAttributes {
    pub pos_comp_type: AttributeType,
    // NOTE: we do not use vector types for attributes because we may want
    // different number of components for some attributes
    pub indices: Vec<MeshIndex>,
    pub positions: Vec<f32>, // 2 or 3 components per position
    pub normals: Vec<f32>, // 3 components per normal
    pub uvs: Vec<f32>, // 2 components per uv
    pub tangents: Vec<f32>, // 3 components per tangent
    pub bitangents: Vec<f32>, //
}


pub mod lightmaps {

    pub use crate::s3tc::Image;

    pub enum LightMaps {
        Basic(Basic),
        NormalMapped(NormalMapped),
    }

    pub struct Basic {
        pub diffuse: Image,
        pub specular: Image,
    }

    pub struct NormalMapped {
        pub diffuse: Image,
        pub specular: Image,
        pub normal: Image,
    }

}

#[derive(Debug)]
pub struct IndexedMesh {
    pub attributes: VertexAttributes,
}

impl IndexedMesh {

    // What if indices exeed the number of positions ?
    // Probably don't need to worry but also be careful !
    pub fn new(attrs: VertexAttributes) -> Self {
        Self {
            attributes: attrs,
        }
    }

    pub fn vertex_count (&self) -> GLuint {
        self.attributes.indices.len() as GLuint
    }

    pub fn index_buffer_size(&self) -> GLsizeiptr {
        (self.attributes.indices.len() * std::mem::size_of::<MeshIndex>()).try_into().unwrap()
    }

    pub fn index_buffer_ptr(&self) -> *const GLvoid {
        self.attributes.indices.as_ptr() as *const GLvoid
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
