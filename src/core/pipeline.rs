use crate::mgl;
use mgl::core::ShaderProgram;
#[macro_use]
use crate::core;
use crate::core::app;
use crate::core::macros;
use std::io;
use std::path::Path;
use std::convert::From;
use gl::types::*;

pub struct Render3D {
    main_shader: mgl::core::ShaderProgram,
}

// pub trait RenderTarget {
   // Render to viewport
   // Render to Texture
   // etc.
// }

mod gpu {

    use std::default::Default;

    pub mod attr_id {
        const POSITION : GLuint = 0;
        const NORMAL : GLuint = 1;
        const UV : GLuint = 3;
    }

    #[derive(Default)]
    pub struct Buffers {
        // Only GLuint allowed
        indices: GLuint,
        position: GLuint,
        normal: GLuint,
        uv: GLuint,
        index: GLuint,
    }

    #[derive(Default)]
    pub struct Textures {
        // Only GLuint allowed
        diffuse: GLuint,
        specular: GLuint,
    }

    #[derive(Default)]
    pub struct Mesh {
        vao: GLuint,
        buffers: Buffers,
    }

    #[derive(Default)]
    pub struct TexturedMesh {
        mesh: Mesh,
        textures: Textures,
    }

    impl Buffers {
        pub fn new() -> Self {
            let buffs = Default::default();
            unsafe {
                gl::GenBuffers((std::mem::size_of(Buffers)/std::size_of(GLuint)) as GLsizei,
                               &mut self as *mut GLuint);
            }
        }
    }

    impl Textures {
        pub fn new() -> Self {
            let texs = Default::default();
            unsafe {
                gl::GenBuffers((std::mem::size_of(Texture)/std::size_of(GLuint)) as GLsizei,
                               &mut self as *mut GLuint);

            }
        }
    }

    impl Mesh {
        pub fn new() -> Self {
            Self {
                vao: 0,
                buffers: Buffers::new(),
            }
        }
    }

    use mgl::attr::mesh3d;

    impl From<mesh3d::IndexedMesh> for Mesh {

        fn from(data: IndexedMesh) -> Self {

            let m: Mesh = Mesh::new();

            // We need to bind index buffers as ELEMENT_BUFFER
            gl::BindBuffer(gl::ELEMENT_BUFFER, m.buffers.indices);
            gl::BufferData(
                gl::ELEMENT_BUFFER,
                data.index_buffer_size(),
                data.index_buffer_ptr(),
                gl::STATIC_DRAW
            )

            gl::BindBuffer(gl::ARRAY_BUFFER, m.buffers.position);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.attributes.position_buffer_size(),
                data.position_buffer_ptr(),
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

            let vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffers.position);
            gl::EnableVertexAttribArray(attr_id::POSITION);

            gl::VertexAttribPointer (
                attr_id::POSITION,
                attrs.pos_comp_type.count(),
                attrs.pos_comp_type.gl_type(),
                0, // tightly-packed
                std::ptr::null()
            );

            gl::EnableVertexAttribArray(attr_id::NORMAL);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffers.normal);
            gl::VertexAttribPointer(
                attr_id::NORMAL,
                3,
                gl::FLOAT,
                gl::TRUE,
                0,
                std::ptr::null()
            );

            gl::EnableVertexAttribArray(attr_id::UV);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffers.uv);
            gl::VertexAttribPointer(
                attr_id::UV,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null()
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        impl TexturedMesh {
            pub fn new(m: Mesh, t: Textures) -> Self {
                Self {
                    mesh: m,
                    textures: t,
                }
            }
        }

    }
}

pub struct Pipeline3D {
    render: Render3D,
    textured_meshes: Vec<gpu::TexturedMesh>
}

#[derive(Debug)]
pub enum InitError {
    ShaderCompileError(String),
    FailedLoadingResource(io::Error),
    ShaderIssue(mgl::core::ShaderIssue)
}

impl_error_conv!(io::Error, InitError, FailedLoadingResource);
impl_error_conv!(mgl::core::ShaderIssue, InitError, ShaderIssue);

fn configure_texture_parameters () {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
}


impl Pipeline3D {

    pub fn create_and_prepare (app: &app::AppCore) -> Result<Self, InitError> {
        Ok( Self {
            render: Render3D {
                main_shader: Self::load_and_compile_shaders(app)?
            }
        })
    }

    pub fn activate_shader(&self) {
        self.render.main_shader.set_active();
    }

    fn configure_gl_parameters(&self) {
        configure_texture_parameters();
    }

    fn prepare_viewport(&self) {
        unsafe {
            gl::Viewport(0, 0, 800, 600);
            gl::Enable(gl::DEPTH_TEST);
        }
    }


    fn load_and_compile_shaders(app: &app::AppCore) -> Result<ShaderProgram, InitError> {

        let vert_shader = mgl::core::Shader::from_source (
            &app.buffer_loader.load_cstring(Path::new("shaders/basic_vert.glsl"))?,
            gl::VERTEX_SHADER
        )?;

        let frag_shader = mgl::core::Shader::from_source (
            &app.buffer_loader.load_cstring(Path::new("shaders/basic_frag.glsl"))?,
            gl::FRAGMENT_SHADER
        )?;

        Ok(mgl::core::ShaderProgram::from_shaders(&[vert_shader, frag_shader])?)
    }
}
