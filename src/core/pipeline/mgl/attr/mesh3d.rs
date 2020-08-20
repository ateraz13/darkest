use gl::types::*;
use std::convert::TryInto;
use super::AttributeType;

type MeshIndex = GLuint;

type Vector3 = cgmath::Vector3<f32>;
type Vector2 = cgmath::Vector2<f32>;

#[derive(Debug)]
pub struct VertexAttributes {
    // NOTE: we do not use vector types for attributes because we may want
    // different number of components for some attributes
    pub indices: Vec<MeshIndex>,
    pub positions: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub tangents: Vec<Vector3>,
    pub bitangents: Vec<Vector3>, //
}

pub mod lightmaps {

    pub use crate::s3tc::Image;

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(attrs: VertexAttributes) -> Self {
        Self {
            attributes: attrs,
        }
    }

    #[allow(dead_code)]
    pub fn vertex_count (&self) -> GLuint {
        self.attributes.indices.len() as GLuint
    }

    #[allow(dead_code)]
    pub fn index_buffer_size(&self) -> GLsizeiptr {
        (self.attributes.indices.len() * std::mem::size_of::<MeshIndex>()).try_into().unwrap()
    }

    #[allow(dead_code)]
    pub fn index_buffer_ptr(&self) -> *const GLvoid {
        self.attributes.indices.as_ptr() as *const GLvoid
    }

    pub fn generate_tangents(&mut self) {

    }

}
