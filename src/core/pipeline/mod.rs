pub mod gpu;
pub mod light_info;
pub mod mgl;

use crate::core::app;
use crate::core::pipeline::mgl::attr::uniform;
use crate::core::pipeline::mgl::shader::ShaderProgram;
use crate::resource::BufferLoaderError;
use std::convert::TryFrom;
// use crate::core::macros;
use crate::cgmath::Array;
use cgmath::prelude::{Matrix, SquareMatrix};
use gl::types::*;
use light_info::DirLight;
use std::convert::From;
use std::path::Path;

// TODO: To optimise the pipeline we should catergorise
// resources into specific groups based on their quality and type.
// This will allows us to do most processing with minimal dynamic dispatch
// and also enable us to replace un-used resources with new ones without
// reallocation.

pub mod resource {
    cenum::enumerate_vals! {
        type ResourceType = u8;
        TEXTURED_MESH = 24, NORMAL_MAPPED_MESH
    }

    // Upper bits 8-bits are resource type identifier
    #[derive(Debug, Clone, Copy)]
    pub struct ResourceID(u32);

    impl ResourceID {
        const U32_MAX: u32 = u32::MAX;
        // const U8_MAX: u8 = u8::MAX;
        const TYPE_PART_MASK: u32 = (u8::MAX as u32) << 24;
        const ID_PART_MASK: u32 = ResourceID::TYPE_PART_MASK ^ ResourceID::U32_MAX;

        pub fn new(rc_type: ResourceType, uid: u32) -> Self {
            if (uid & Self::TYPE_PART_MASK) != 0 {
                panic!("Invalid ResourceID created! (ID exceeds 24-bit bounds)");
            }

            // Uppper 8 bits identify resource type the rest is a unique identifier
            Self(((rc_type.0 as u32) << 24) | (uid))
        }

        pub fn get_type(&self) -> ResourceType {
            ResourceType((self.0 >> 24) as u8)
        }

        pub fn as_index(&self) -> usize {
            (Self::ID_PART_MASK & self.0) as usize
        }
    }
}

use resource::ResourceID;

pub struct Render3D {
    main_shader: ShaderProgram,
    model_mat_unif: uniform::Mat4Uniform,
    view_mat_unif: uniform::Mat4Uniform,
    modelview_mat_unif: uniform::Mat4Uniform,
    proj_mat_unif: uniform::Mat4Uniform,
    mvp_mat_unif: uniform::Mat4Uniform,
    normal_mat_unif: uniform::Mat4Uniform,
    // view_pos_unif: uniform::Vec3Uniform,
    // time_unif: uniform::FloatUniform,
    use_normalmap_unif: uniform::BoolUniform,
    sun_intensity_unif: uniform::FloatUniform,
    sun_direction_unif: uniform::Vec3Uniform,
    sun_ambient_unif: uniform::Vec3Uniform,
    sun_diffuse_unif: uniform::Vec3Uniform,
    sun_specular_unif: uniform::Vec3Uniform,
    lamp_position_unif: uniform::Vec3Uniform,
    lamp_ambient_unif: uniform::Vec3Uniform,
    lamp_diffuse_unif: uniform::Vec3Uniform,
    lamp_specular_unif: uniform::Vec3Uniform,
}

type Mat4 = cgmath::Matrix4<f32>;
// type Vec3 = cgmath::Vector3<f32>;
type Point3 = cgmath::Point3<f32>;

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
    sun: DirLight,
    view_pos: Point3,
}

#[derive(Debug)]
pub enum InitError {
    FailedLoadingResource(BufferLoaderError),
    ShaderIssue(mgl::shader::ShaderIssue),
}

impl_error_conv!(BufferLoaderError, InitError, FailedLoadingResource);
impl_error_conv!(mgl::shader::ShaderIssue, InitError, ShaderIssue);

fn configure_texture_parameters() {
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::MIRRORED_REPEAT as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
}

impl Pipeline3D {
    pub fn create_and_prepare(app: &app::AppCore) -> Result<Self, InitError> {
        let main_shader = Self::load_and_compile_shader(app)?;
        let get_unif = |name| {
            if let Some(unif) = main_shader.uniform_by_name(name) {
                println!("Found uniform name: {}", name);
                return unif;
            } else {
                panic!("Could find uniform called {}.", name);
            }
        };

        let uMat4 = |name| uniform::Mat4Uniform::try_from(get_unif(name));
        let uVec3 = |name| uniform::Vec3Uniform::try_from(get_unif(name));
        let uBool = |name| uniform::BoolUniform::try_from(get_unif(name));
        let uFloat = |name| uniform::FloatUniform::try_from(get_unif(name));

        let p3d = Self {
            render: Render3D {
                model_mat_unif: uMat4("model_mat")?,
                view_mat_unif: uMat4("view_mat")?,
                modelview_mat_unif: uMat4("modelview_mat")?,
                proj_mat_unif: uMat4("proj_mat")?,
                mvp_mat_unif: uMat4("mvp_mat")?,
                normal_mat_unif: uMat4("normal_mat")?,
                // FIXME: The time uniform cannot be requested for some reasons.
                // time_unif: uFloat("time")?,
                use_normalmap_unif: uBool("use_normalmap")?,
                sun_intensity_unif: uFloat("sun.intensity")?,
                sun_direction_unif: uVec3("sun.direction")?,
                sun_ambient_unif: uVec3("sun.ambient")?,
                sun_diffuse_unif: uVec3("sun.diffuse")?,
                sun_specular_unif: uVec3("sun.specular")?,
                lamp_position_unif: uVec3("lamp.position")?,
                lamp_ambient_unif: uVec3("lamp.ambient")?,
                lamp_diffuse_unif: uVec3("lamp.diffuse")?,
                lamp_specular_unif: uVec3("lamp.specular")?,
                main_shader: main_shader,
            },
            projection_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            basic_tex_meshes: vec![],
            normal_mapped_tex_meshes: vec![],
            view_pos: cgmath::Point3::<f32>::new(0.0f32, 0.0, 0.0),
            sun: DirLight::default(),
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
    #[allow(dead_code)]
    pub fn prepare_basic_textured_meshes(
        &mut self,
        data: &[(
            &mgl::attr::mesh3d::lightmaps::Basic,
            &mgl::attr::mesh3d::IndexedMesh,
        )],
    ) -> Vec<ResourceID> {
        let mut ids: Vec<ResourceID> = vec![];
        ids.reserve(data.len());

        self.basic_tex_meshes.clear();
        self.basic_tex_meshes.reserve_exact(data.len());
        for (i, (lm, im)) in data.iter().enumerate() {
            let mut tm = gpu::basic_mesh::Mesh::from(*im);
            // println!("TEXTURED MESH CREATED: {:?}", tm);
            tm.textures.upload_all_textures(&lm);

            ids.push(ResourceID::new(resource::TEXTURED_MESH, i as u32));

            self.basic_tex_meshes.push(mesh_data::Basic {
                resource: tm,
                model_matrix: Mat4::identity(),
                normal_matrix: Mat4::identity(),
            });
        }
        ids
    }

    pub fn prepare_normal_mapped_textured_meshes(
        &mut self,
        data: &[(
            &mgl::attr::mesh3d::lightmaps::NormalMapped,
            &mgl::attr::mesh3d::IndexedMesh,
        )],
    ) -> Vec<ResourceID> {
        let mut ids: Vec<ResourceID> = vec![];
        ids.reserve(data.len());

        self.basic_tex_meshes.clear();
        self.basic_tex_meshes.reserve_exact(data.len());
        for (i, (lm, im)) in data.iter().enumerate() {
            let mut tm = gpu::normal_mapped_mesh::Mesh::from(*im);
            // println!("NORMAL MAPPED MESH CREATED: {:?}", tm);
            tm.textures.upload_all_textures(&lm);

            println!("I: {}", i);

            let new_id = ResourceID::new(resource::NORMAL_MAPPED_MESH, i as u32);

            ids.push(new_id);
            self.normal_mapped_tex_meshes.push(mesh_data::NormalMapped {
                resource: tm,
                model_matrix: Mat4::identity(),
                normal_matrix: Mat4::identity(),
            });
        }

        ids
    }

    fn load_and_compile_shader(app: &app::AppCore) -> Result<ShaderProgram, InitError> {
        let vert_shader = mgl::shader::Shader::from_source(
            &app.buffer_loader
                .load_cstring(Path::new("shaders/basic_vert.glsl"))?,
            gl::VERTEX_SHADER,
        )?;

        let frag_shader = mgl::shader::Shader::from_source(
            &app.buffer_loader
                .load_cstring(Path::new("shaders/basic_frag.glsl"))?,
            gl::FRAGMENT_SHADER,
        )?;

        Ok(mgl::shader::ShaderProgram::from_shaders(&[
            vert_shader,
            frag_shader,
        ])?)
    }

    pub fn update_model_matrix(&mut self, id: ResourceID, mat: Mat4) {
        match id.get_type() {
            resource::TEXTURED_MESH => self.basic_tex_meshes[id.as_index()].model_matrix = mat,
            resource::NORMAL_MAPPED_MESH => {
                self.normal_mapped_tex_meshes[id.as_index()].model_matrix = mat
            }
            _ => {}
        }
    }

    pub fn update_normal_matrix(&mut self, id: ResourceID, mat: Mat4) {
        match id.get_type() {
            resource::TEXTURED_MESH => self.basic_tex_meshes[id.as_index()].normal_matrix = mat,
            resource::NORMAL_MAPPED_MESH => {
                self.normal_mapped_tex_meshes[id.as_index()].normal_matrix = mat
            }
            _ => {}
        }
    }

    pub fn update_view_matrix(&mut self, mat: Mat4) {
        self.view_matrix = mat;
    }

    pub fn update_projection_matrix(&mut self, mat: Mat4) {
        self.projection_matrix = mat;
    }

    pub fn update_view_pos(&mut self, pos: Point3) {
        self.view_pos = pos.clone();
    }

    pub fn draw_textured_meshes(&self) {
        // disable normal maps
        unsafe {
            gl::Uniform1ui(self.render.use_normalmap_unif.def.id, 0);
            self.render.main_shader.set_active();
            gl::Uniform1i(
                gpu::attrs::DIFFUSE_SAMPLER_LOCATION,
                gpu::attrs::DIFFUSE_TEXTURE_UNIT as i32,
            ); // Texture Unit 0 : DIFFUSE
            gl::Uniform1i(
                gpu::attrs::SPECULAR_SAMPLER_LOCATION,
                gpu::attrs::SPECULAR_TEXTURE_UNIT as i32,
            ); // Texture Unit 1 : SPECULAR
            gl::Uniform1i(
                gpu::attrs::NORMAL_SAMPLER_LOCATION,
                gpu::attrs::NORMAL_TEXTURE_UNIT as i32,
            ); // Texture Unit 2 : NORMAL
            gl::Uniform3fv(
                gpu::attrs::uniforms::VIEW_POS_LOCATION,
                1,
                self.view_pos.as_ptr(),
            );
        }

        for m in self.basic_tex_meshes.iter() {
            unsafe {
                let mv = self.view_matrix * m.model_matrix;
                let mvp = self.projection_matrix * mv;

                gl::UniformMatrix4fv(
                    self.render.model_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    m.model_matrix.as_ptr(),
                );
                gl::UniformMatrix4fv(
                    self.render.modelview_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    mv.as_ptr(),
                );
                gl::UniformMatrix4fv(
                    self.render.proj_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    self.projection_matrix.as_ptr(),
                );
                gl::UniformMatrix4fv(self.render.mvp_mat_unif.def.id, 1, gl::FALSE, mvp.as_ptr());
                gl::UniformMatrix4fv(
                    self.render.normal_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    m.normal_matrix.as_ptr(),
                );
            }

            self.render.draw(&m.resource);
        }

        // enable normal maps
        // unsafe {
        //     gl::Uniform1ui(gpu::attrs::USE_NORMALMAP_FLAG, 1);
        // }

        pub use gpu::attrs::uniforms;

        for m in self.normal_mapped_tex_meshes.iter() {
            unsafe {
                let mv = self.view_matrix * m.model_matrix;
                let mvp = self.projection_matrix * mv;

                gl::UniformMatrix4fv(
                    self.render.model_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    m.model_matrix.as_ptr(),
                );
                gl::UniformMatrix4fv(
                    self.render.view_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    self.view_matrix.as_ptr(),
                );
                gl::UniformMatrix4fv(
                    self.render.modelview_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    mv.as_ptr(),
                );
                gl::UniformMatrix4fv(
                    self.render.proj_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    self.projection_matrix.as_ptr(),
                );
                gl::UniformMatrix4fv(self.render.mvp_mat_unif.def.id, 1, gl::FALSE, mvp.as_ptr());
                gl::UniformMatrix4fv(
                    self.render.normal_mat_unif.def.id,
                    1,
                    gl::FALSE,
                    m.normal_matrix.as_ptr(),
                );

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
                0 as *const GLvoid,
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
                0 as *const GLvoid,
            );
        }
    }
}
