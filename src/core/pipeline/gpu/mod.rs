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

macro_rules! define_buffers {
    {$name:ident { $first_field:ident, $($other_fields:ident),+} } => {

        #[derive(Default,Debug)]
        struct $name {
            pub $first_field : IdVal,
            $(pub $other_fields : IdVal,)*
        }

        impl $name {
            fn new () -> Self {
                let buffs = Self {
                    $first_field : 0,
                    $($other_fields : 0,)*
                };

                unsafe {
                    gl::GenBuffers((std::mem::size_of::<Self>()/std::mem::size_of::<IdVal>()) as GLsizei,
                                   (&mut buffs.$first_field) as *mut GLuint);
                }

                buffs
            }

            unsafe fn as_ptr(&self) -> *const IdVal {
                &self.$first_field as *const IdVal
            }

            unsafe fn as_mut_ptr(&mut self) -> *mut IdVal {
                &mut self.$first_field as *mut IdVal
            }

            fn print_ids(&self) {
                println!("struct {}", stringify!($name));
                println!("\t{} : {}", stringify!($first_field), self.$first_field);
                $(println!("\t{} : {}", stringify!($other_fields), self.$other_fields);)+
                    println!("}}");
            }
        }
    }


}

pub mod basic_mesh {

    use super::{ textures, IdVal, attrs };
    use gl::types::*;
    use crate::core::pipeline::mgl::attr::mesh3d;

    define_buffers!( Buffers {
        index, position, normal, uv
    });

    #[derive(Debug)]
    pub struct Mesh {
        pub vao: IdVal,
        pub element_count: GLsizei,
        pub buffers: Buffers,
        pub textures: textures::Basic,
    }

    impl Mesh {
        pub fn new() -> Self {
            Self {
                vao: 0,
                element_count: 0,
                buffers: Buffers::new(),
                textures: textures::Basic::new(),
            }
        }
    }

    fn size_of_vec<T>(v: &Vec<T>) -> GLsizeiptr {
        ( std::mem::size_of::<T>() * v.len() ) as GLsizeiptr
    }


    #[allow(unused_macros)]
    macro_rules! buffer_bind_target {
        (element_array) => {gl::ELEMENT_ARRAY_BUFFER};
        (array) => {gl::ARRAY_BUFFER};
    }

    macro_rules! vertex_attrib_type {
        (float) => {gl::FLOAT};
        (vec2) => {gl::FLOAT};
        (vec3) => {gl::FLOAT};
        (vec4) => {gl::FLOAT};
        (mat2) => {gl::FLOAT};
        (mat3) => {gl::FLOAT};
        (mat4) => {gl::FLOAT};
    }

    macro_rules! vertex_attrib_component_count {
        (float) => {1};
        (vec2) => {2};
        (vec3) => {3};
        (vec4) => {4};
        (mat2) => {4};
        (mat3) => {9};
        (mat4) => {16};
    }

    macro_rules! vertex_attrib_ptr {
        (target: element_array $($rest:tt)*) => {};
        (target: $target:tt, id: $id:expr, location: $loc:expr, config: packed $type:tt array) => {
            gl::BindBuffer(buffer_bind_target!($target), $id);
            gl::EnableVertexAttribArray($loc);

            gl::VertexAttribPointer(
                $loc,
                vertex_attrib_component_count!($type),
                vertex_attrib_type!($type),
                gl::FALSE,
                0,
                std::ptr::null()
            );
        }
    }

    #[allow(unused_macros)]
    macro_rules! buffer_access_method {
        (static_draw) => {gl::STATIC_DRAW};
    }

    macro_rules! buffer_data  {
        (generate_vao; $(($id:expr) => { data: $data:expr, target: $target:tt, access: $access:tt$(, $($rest:tt)*)? }),+) => {{
            $(
                gl::BindBuffer(buffer_bind_target!($target), $id);
                gl::BufferData(
                    buffer_bind_target!($target),
                    size_of_vec(&$data),
                    $data.as_ptr() as *const GLvoid,
                    buffer_access_method!($access)
                );
            )+

            let mut vao;
            gl::GenVertexArrays(1, &mut vao);

            $(
                vertex_attrib_ptr!(
                    target: $target, $(id: $id,$($rest)*)?
                );
            )+
                vao
        }}
    }

    use std::convert::TryInto;

    impl From<&mesh3d::IndexedMesh> for Mesh {

        fn from(data: &mesh3d::IndexedMesh) -> Self {

            let mut mesh: Mesh = Mesh::new();

            m.element_count = data.attributes.indices.len().try_into().unwrap();

            unsafe {

                m.vao = buffer_data!(
                    generate_vao;

                    (mesh.buffers.index) => {
                        data: data.attributes.indices,
                        target: element_array,
                        access: static_draw
                    },
                    (mesh.buffers.position) => {
                        data: data.attributes.positions,
                        target: array,
                        access: static_draw,
                        location: attrs::POSITION_LOCATION,
                        config: packed vec3 array
                    },
                    (mesh.buffers.normal) => {
                        data: data.attributes.normals,
                        target: array,
                        access: static_draw,
                        location: attrs::NORMAL_LOCATION,
                        config: packed vec3 array
                    },
                    (mesh.buffers.uv) => {
                        data: data.attributes.uvs,
                        target: array,
                        access: static_draw,
                        location: attrs::UV_LOCATION,
                        config: packed vec2 array
                    }
                );


            mesh
        }
    }
}

pub mod normal_mapped_mesh {

    use super::textures;
    use super::IdVal;
    use gl::types::*;

    define_buffers!( Buffers {
        index, position, normal, uv, tangent, bitangent
    });

    pub struct Mesh {
        pub vao: IdVal,
        pub element_count: GLsizei,
        pub buffers: Buffers,
        pub textures: textures::NormalMapped,
    }

    impl Mesh {
        pub fn new() -> Self {
            Self {
                vao: 0,
                element_count: 0,
                buffers: Buffers::new(),
                textures: textures::NormalMapped::new(),
            }
        }
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
