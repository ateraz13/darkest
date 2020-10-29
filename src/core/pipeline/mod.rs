pub mod mgl;
pub mod gpu;

use crate::resource::BufferLoaderError;
use crate::core::app;
use crate::core::pipeline::mgl::shader::ShaderProgram;
// use crate::core::macros;
use std::path::Path;
use std::convert::From;
use gl::types::*;
use cgmath::prelude::{ Matrix, SquareMatrix };

// TODO: To optimise the pipeline we should catergorise
// resources into specific groups based on their quality and type.
// This will allows us to do most processing with minimal dynamic dispatch
// and also enable us to replace un-used resources with new ones without
// reallocation.

pub mod resource {
    cenum::enumerate_vals! {
        type ResourceType = u8;
        TEXTURED_MESH, NORMAL_MAPPED_MESH
    }

    // Upper bits 8-bits are resource type identifier
    #[derive(Debug, Clone, Copy)]
    pub struct ResourceID (u32);

    impl ResourceID {
        const U32_MAX : u32 =  u32::MAX;
        const U8_MAX : u8 = u8::MAX;
        const TYPE_PART_MASK: u32 = (ResourceID::U8_MAX as u32) << 24;
        const ID_PART_MASK: u32 = ResourceID::TYPE_PART_MASK ^ ResourceID::U32_MAX;

        pub fn new(rc_type: ResourceType, uid: u32) -> Self {

            if ( uid & Self::TYPE_PART_MASK ) != 0 {
                panic!("Invalid ResourceID created! (ID exceeds 24-bit bounds)");
            }

            // Uppper 8 bits identify resource type the rest is a unique identifier
            Self( (rc_type.0 as u32) << 24 & (uid | ResourceID::TYPE_PART_MASK) )
        }

        pub fn get_type(&self) -> ResourceType {
            ResourceType(( self.0 >> 24 ) as u8)
        }

        pub fn as_index(&self) -> usize {
            ( Self::ID_PART_MASK & self.0 ) as usize
        }

    }
}

use resource::ResourceID;

pub struct Render3D {
    main_shader: ShaderProgram,
}

type Mat4 = cgmath::Matrix4<f32>;

pub mod mesh_data {

    use super::gpu;
    use super::Mat4;

    #[derive(Debug)]
    pub struct Basic {
        pub resource: gpu::basic_mesh::Mesh,
        pub model_matrix: Mat4,
        pub normal_matrix: Mat4,
    }

    #[derive(Debug)]
    pub struct NormalMapped {
        pub resource: gpu::normal_mapped_mesh::Mesh,
        pub model_matrix: Mat4,
        pub normal_matrix: Mat4,
    }
}

pub struct Pipeline3D {
    render: Render3D,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    basic_tex_meshes: Vec<mesh_data::Basic>,
    normal_mapped_tex_meshes: Vec<mesh_data::NormalMapped>,
}

#[derive(Debug)]
pub enum InitError {
    FailedLoadingResource(BufferLoaderError),
    ShaderIssue(mgl::shader::ShaderIssue)
}

impl_error_conv!(BufferLoaderError, InitError, FailedLoadingResource);
impl_error_conv!(mgl::shader::ShaderIssue, InitError, ShaderIssue);

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
        let p3d =  Self {
            render: Render3D {
                main_shader: Self::load_and_compile_shader(app)?
            },
            projection_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            basic_tex_meshes: vec![],
            normal_mapped_tex_meshes: vec![]
        };

        p3d.configure_gl_parameters();
        p3d.prepare_viewport();

        Ok(p3d)
    }

    pub fn activate_shader(&self) {
        self.render.main_shader.set_active();
    }

    fn configure_gl_parameters(&self) {
        configure_texture_parameters();
    }

    fn prepare_viewport(&self) {
        unsafe {
            gl::Viewport(0, 0, 1024, 768);
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    //FIXME: If this is slow than try something else
    pub fn prepare_basic_textured_meshes(&mut self, data: &[(&mgl::attr::mesh3d::lightmaps::Basic, &mgl::attr::mesh3d::IndexedMesh)]) -> Vec<ResourceID>
    {
        let mut ids : Vec<ResourceID> = vec![];
        ids.reserve(data.len());

        self.basic_tex_meshes.clear();
        self.basic_tex_meshes.reserve_exact(data.len());
        for (lm, im) in data.iter() {
            let mut tm = gpu::basic_mesh::Mesh::from(*im);
            // println!("TEXTURED MESH CREATED: {:?}", tm);
            tm.textures.upload_all_textures(&lm);

            ids.push(ResourceID::new (
                resource::TEXTURED_MESH,
                self.basic_tex_meshes.len() as u32)
            );

            self.basic_tex_meshes.push(mesh_data::Basic {
                resource: tm,
                model_matrix: Mat4::identity(),
                normal_matrix: Mat4::identity()
            });

        }
        ids
    }

    pub fn prepare_normal_mapped_textured_meshes (&mut self, data: &[(&mgl::attr::mesh3d::lightmaps::NormalMapped, &mgl::attr::mesh3d::IndexedMesh)]) -> Vec<ResourceID>
    {
        let mut ids : Vec<ResourceID> = vec![];
        ids.reserve(data.len());

        self.basic_tex_meshes.clear();
        self.basic_tex_meshes.reserve_exact(data.len());
        for (lm, im) in data.iter() {
            let mut tm = gpu::normal_mapped_mesh::Mesh::from(*im);
            // println!("NORMAL MAPPED MESH CREATED: {:?}", tm);
            tm.textures.upload_all_textures(&lm);

            ids.push(ResourceID::new (
                resource::NORMAL_MAPPED_MESH,
                self.basic_tex_meshes.len() as u32)
            );

            self.normal_mapped_tex_meshes.push( mesh_data::NormalMapped {
                resource: tm,
                model_matrix: Mat4::identity(),
                normal_matrix: Mat4::identity()
            });
        }

        ids
    }

    fn load_and_compile_shader(app: &app::AppCore) -> Result<ShaderProgram, InitError> {

        let vert_shader = mgl::shader::Shader::from_source (
            &app.buffer_loader.load_cstring(Path::new("shaders/basic_vert.glsl"))?,
            gl::VERTEX_SHADER
        )?;

        let frag_shader = mgl::shader::Shader::from_source (
            &app.buffer_loader.load_cstring(Path::new("shaders/basic_frag.glsl"))?,
            gl::FRAGMENT_SHADER
        )?;

        Ok(mgl::shader::ShaderProgram::from_shaders(&[vert_shader, frag_shader])?)
    }

    pub fn update_model_matrix(&mut self, id: ResourceID, mat: Mat4) {
        match id.get_type() {
            resource::TEXTURED_MESH => {
                self.basic_tex_meshes[id.as_index()].model_matrix = mat
            },
            resource::NORMAL_MAPPED_MESH => {
                self.normal_mapped_tex_meshes[id.as_index()].model_matrix = mat
            },
            _ => {}
        }
    }

    pub fn update_normal_matrix(&mut self, id: ResourceID, mat: Mat4) {
        match id.get_type() {
            resource::TEXTURED_MESH => {
                self.basic_tex_meshes[id.as_index()].normal_matrix = mat
            },
            resource::NORMAL_MAPPED_MESH => {
                self.normal_mapped_tex_meshes[id.as_index()].normal_matrix = mat
            },
            _ => {}
        }
    }

    pub fn update_view_matrix(&mut self, mat: Mat4) {
        self.view_matrix = mat;
    }

    pub fn update_projection_matrix(&mut self, mat: Mat4) {
        self.projection_matrix = mat;
    }

    pub fn draw_textured_meshes(&self) {

        // disable normal maps
        unsafe {
            gl::Uniform1ui(gpu::attrs::USE_NORMALMAP_FLAG, 0);
            self.render.main_shader.set_active();
            gl::Uniform1i(gpu::attrs::DIFFUSE_SAMPLER_LOCATION,  gpu::attrs::DIFFUSE_TEXTURE_UNIT as i32); // Texture Unit 0 : DIFFUSE
            gl::Uniform1i(gpu::attrs::SPECULAR_SAMPLER_LOCATION, gpu::attrs::SPECULAR_TEXTURE_UNIT as i32); // Texture Unit 1 : SPECULAR
            gl::Uniform1i(gpu::attrs::NORMAL_SAMPLER_LOCATION,   gpu::attrs::NORMAL_TEXTURE_UNIT as i32); // Texture Unit 2 : NORMAL
        }

        for m in self.basic_tex_meshes.iter() {
            unsafe {
                let mv = self.view_matrix*m.model_matrix;
                let mvp = self.projection_matrix * mv;

                gl::UniformMatrix4fv(1, 1, gl::FALSE, m.model_matrix.as_ptr());
                gl::UniformMatrix4fv(2, 1, gl::FALSE, mv.as_ptr());
                gl::UniformMatrix4fv(3, 1, gl::FALSE, self.projection_matrix.as_ptr());
                gl::UniformMatrix4fv(4, 1, gl::FALSE, mvp.as_ptr());
                gl::UniformMatrix4fv(5, 1, gl::FALSE, m.normal_matrix.as_ptr());

            }

            self.render.draw(&m.resource);
        }

        // enable normal maps
        unsafe {
            gl::Uniform1ui(gpu::attrs::USE_NORMALMAP_FLAG, 1);
        }

        pub use gpu::attrs::uniforms;

        for m in self.normal_mapped_tex_meshes.iter() {
            unsafe {
                let mv = self.view_matrix*m.model_matrix;
                let mvp = self.projection_matrix * mv;

                gl::UniformMatrix4fv(uniforms::MODEL_MAT_LOCATION, 1, gl::FALSE, m.model_matrix.as_ptr());
                gl::UniformMatrix4fv(uniforms::VIEW_MAT_LOCATION, 1, gl::FALSE, self.view_matrix.as_ptr());
                gl::UniformMatrix4fv(uniforms::MODELVIEW_MAT_LOCATION, 1, gl::FALSE, mv.as_ptr());
                gl::UniformMatrix4fv(uniforms::PROJECTION_MAT_LOCATION, 1, gl::FALSE, self.projection_matrix.as_ptr());
                gl::UniformMatrix4fv(uniforms::MVP_MAT_LOCATION, 1, gl::FALSE, mvp.as_ptr());
                gl::UniformMatrix4fv(uniforms::NORMAL_MAT_LOCATION, 1, gl::FALSE, m.normal_matrix.as_ptr());

                self.render.main_shader.set_active();
            }

            self.render.draw(&m.resource);
        }
    }
}

trait Draw<T> {
    fn draw(&self, e: &T);
}

impl Draw<gpu::basic_mesh::Mesh> for Render3D {

    fn draw(&self, e: &gpu::basic_mesh::Mesh) {

        unsafe {

            gl::ActiveTexture(gl::TEXTURE0 + gpu::attrs::DIFFUSE_TEXTURE_UNIT);
            gl::BindTexture(gl::TEXTURE_2D, e.textures.diffuse);
            gl::ActiveTexture(gl::TEXTURE0 + gpu::attrs::SPECULAR_TEXTURE_UNIT);
            gl::BindTexture(gl::TEXTURE_2D, e.textures.specular);

            gl::BindVertexArray(e.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, e.buffers.index);
            gl::DrawElements(
                gl::TRIANGLES,
                e.element_count,
                gl::UNSIGNED_INT,
                0 as *const GLvoid
            );

        }
    }

}

impl Draw<gpu::normal_mapped_mesh::Mesh> for Render3D {

    fn draw(&self, e: &gpu::normal_mapped_mesh::Mesh) {

        unsafe {

            gl::ActiveTexture(gl::TEXTURE0 + gpu::attrs::DIFFUSE_TEXTURE_UNIT);
            gl::BindTexture(gl::TEXTURE_2D, e.textures.diffuse);
            gl::ActiveTexture(gl::TEXTURE0 + gpu::attrs::SPECULAR_TEXTURE_UNIT);
            gl::BindTexture(gl::TEXTURE_2D, e.textures.specular);
            gl::ActiveTexture(gl::TEXTURE0 + gpu::attrs::NORMAL_TEXTURE_UNIT);
            gl::BindTexture(gl::TEXTURE_2D, e.textures.normal);

            gl::BindVertexArray(e.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, e.buffers.index);
            gl::DrawElements(
                gl::TRIANGLES,
                e.element_count,
                gl::UNSIGNED_INT,
                0 as *const GLvoid
            );

        }
    }

}
