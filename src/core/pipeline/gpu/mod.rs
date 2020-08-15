pub mod textures;

// use std::convert::TryInto;
// use crate::core::pipeline::mgl::s3tc::Image;
use gl::types::*;
use std::default::Default;
use textures::Textures;

pub type IdVal = GLuint;

// attribute and uniform locations
pub mod attrs {

    pub type IdVal = gl::types::GLuint;

    pub const POSITION_LOCATION : IdVal = 0;
    pub const NORMAL_LOCATION : IdVal = 1;
    pub const UV_LOCATION : IdVal = 2;

    pub const DIFFUSE_TEXTURE_UNIT : IdVal = gl::TEXTURE0;
    pub const DIFFUSE_SAMPLER_LOCATION : IdVal = 7;

    pub const SPECULAR_TEXTURE_UNIT : IdVal = gl::TEXTURE1;
    pub const SPECULAR_SAMPLER_LOCATION : IdVal = 8;

    pub const NORMAL_TEXTURE_UNIT : IdVal = gl::TEXTURE2;
    pub const NORMAL_SAMPLER_LOCATION : IdVal = 9;
}

#[derive(Default,Debug)]
pub struct Buffers {
    // Only id values allowed
    pub indices: IdVal,
    pub position: IdVal,
    pub normal: IdVal,
    pub uv: IdVal,
}

#[derive(Default,Debug)]
pub struct Mesh {
    pub vao: IdVal,
    pub element_count: GLsizei,
    pub buffers: Buffers,
}

#[derive(Debug)]
pub struct TexturedMesh {
    pub mesh: Mesh,
    pub textures: textures::Textures,
}

impl Buffers {
    pub fn new() -> Self {
        let mut buffs : Buffers = Default::default();
        unsafe {
            gl::GenBuffers((std::mem::size_of::<Buffers>()/std::mem::size_of::<IdVal>()) as GLsizei,
                           (&mut buffs.indices) as *mut GLuint);
        }
        buffs
    }
}



impl Mesh {
    pub fn new() -> Self {
        Self {
            vao: 0,
            element_count: 0,
            buffers: Buffers::new(),
        }
    }
}

use mgl::attr::mesh3d;

impl From<&mesh3d::IndexedMesh> for Mesh {

    fn from(data: &mesh3d::IndexedMesh) -> Self {

        let mut m: Mesh = Mesh::new();

        m.element_count = data.indices.len().try_into().unwrap();

        unsafe {

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, m.buffers.indices);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                data.index_buffer_size(),
                data.index_buffer_ptr(),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, m.buffers.position);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.attributes.position_buffer_size(),
                data.attributes.position_buffer_ptr(),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, m.buffers.normal);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.attributes.normal_buffer_size(),
                data.attributes.normal_buffer_ptr(),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, m.buffers.uv);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.attributes.uv_buffer_size(),
                data.attributes.uv_buffer_ptr(),
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind

            gl::GenVertexArrays(1, &mut m.vao);

            gl::BindVertexArray(m.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, m.buffers.position);
            gl::EnableVertexAttribArray(attrs::POSITION_LOCATION);

            gl::VertexAttribPointer (
                attrs::POSITION_LOCATION,
                3,// data.attributes.pos_comp_type.count() as i32,
                gl::FLOAT,// data.attributes.pos_comp_type.gl_type(),
                gl::FALSE,
                0, // tightly-packed
                std::ptr::null()
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, m.buffers.normal);
            gl::EnableVertexAttribArray(attrs::NORMAL_LOCATION);

            gl::VertexAttribPointer(
                attrs::NORMAL_LOCATION,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null()
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, m.buffers.uv);
            gl::EnableVertexAttribArray(attrs::UV_LOCATION);

            gl::VertexAttribPointer(
                attrs::UV_LOCATION,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null()
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

        }
        return m;
    }
}

impl TexturedMesh {
    pub fn new(m: Mesh, t: Textures) -> Self {
        Self {
            mesh: m,
            textures: t,
        }
    }
}
