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

        let indices = &self.attributes.indices;
        let pos = &self.attributes.positions;
        let uvs = &self.attributes.uvs;
        let tans = &mut self.attributes.tangents;
        let bitans = &mut self.attributes.bitangents;

        tans.clear();
        bitans.clear();

        tans.reserve_exact(pos.len());
        bitans.reserve_exact(pos.len());


        for i in (0 .. indices.len()).step_by(3) {

            let (v1, v2, v3) = (
                &pos[indices[i] as usize],
                &pos[indices[ i+1 ] as usize],
                &pos[indices[ i+2 ] as usize]
            );

            let (uv1, uv2, uv3) = (
                &uvs[indices[i] as usize],
                &uvs[indices[ i+1 ] as usize],
                &uvs[indices[ i+2 ] as usize]
            );

            let delta_pos1 = v2 - v1;
            let delta_pos2 = v3 - v1;

            let delta_uv1 = uv2 - uv1;
            let delta_uv2 = uv3 - uv1;

            let r = 1.0f32 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
            let tan = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y)*r;
            let bitan = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x)*r;

            for _ in 0 .. 3 {
                tans.push(tan);
                bitans.push(bitan);
            }

        }

    }

}
