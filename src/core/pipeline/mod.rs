pub mod mgl;
pub mod gpu;

use mgl::shader::ShaderProgram;
use crate::core::app;
// use crate::core::macros;
use std::io;
use std::path::Path;
use std::convert::From;
use crate::resource::BufferLoaderError;
use gl::types::*;


// TODO: To optimise the pipeline we should catergorise
// resources into specific groups based on their quality and type.
// This will allows us to do most processing with minimal dynamic dispatch
// and also enable us to replace un-used resources with new ones without
// reallocation.

pub struct Render3D {
    main_shader: ShaderProgram,
}

type Mat4 = na::Matrix4<f32>;

#[derive(Debug)]
pub struct TexMeshData {
    pub resource: gpu::TexturedMesh,
    pub model_matrix: Mat4,
    pub normal_matrix: Mat4,
}

pub struct Pipeline3D {
    render: Render3D,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    tex_meshes: Vec<TexMeshData>,
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
            tex_meshes: vec![],
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
            gl::Viewport(0, 0, 800, 600);
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    //FIXME: If this is slow than try something else
    pub fn prepare_textured_meshes(&mut self, data: &[(&mgl::attr::mesh3d::LightMaps, &mgl::attr::mesh3d::IndexedMesh)])
    {
        self.tex_meshes.clear();
        self.tex_meshes.reserve_exact(data.len());
        for (lm, im) in data.iter() {
            let tm = gpu::TexturedMesh::new(gpu::Mesh::from(*im),
                                             gpu::Textures::from(*lm));
            self.tex_meshes.push(TexMeshData{
                resource: tm,
                model_matrix: Mat4::identity(),
                normal_matrix: Mat4::identity()
            });
        }
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

    pub fn update_model_matrix(&mut self, id: u32, mat: Mat4) {
        self.tex_meshes[id as usize].model_matrix = mat
    }

    pub fn update_normal_matrix(&mut self, id: u32, mat: Mat4) {
        self.tex_meshes[id as usize].normal_matrix = mat
    }

    // There may be problems with multiplying the view and model matrices
    pub fn update_view_matrix(&mut self, mat: Mat4) {
        self.view_matrix = mat;
        // unsafe {
        //     gl::UniformMatrix4fv(4, 1, gl::FALSE, mat.as_ptr())
        // }
    }

    pub fn update_projection_matrix(&mut self, mat: Mat4) {
        self.projection_matrix = mat;
        // unsafe {
        //     gl::UniformMatrix4fv(4, 1, gl::FALSE, mat.as_ptr())
        // }
    }

    pub fn draw_textured_meshes(&self) {
        for m in self.tex_meshes.iter() {
            unsafe {
                let mv = self.view_matrix*m.model_matrix;
                let mvp = self.projection_matrix * mv;

                gl::UniformMatrix4fv(1, 1, gl::FALSE, m.model_matrix.as_ptr());
                gl::UniformMatrix4fv(2, 1, gl::FALSE, mv.as_ptr());
                gl::UniformMatrix4fv(3, 1, gl::FALSE, self.projection_matrix.as_ptr());
                gl::UniformMatrix4fv(4, 1, gl::FALSE, mvp.as_ptr());
                gl::UniformMatrix4fv(5, 1, gl::FALSE, m.normal_matrix.as_ptr());

                self.render.main_shader.set_active();
            }

            // println!("DRAWING: {:?}", m);

            self.render.draw(&m.resource);
        }
    }
}

trait Draw<T> {
    fn draw(&self, e: &T);
}

impl Draw<gpu::TexturedMesh> for Render3D {

    fn draw(&self, e: &gpu::TexturedMesh) {

        unsafe {

            gl::ActiveTexture(gpu::attrs::DIFFUSE_TEXTURE_UNIT);
            gl::BindTexture(gl::TEXTURE_2D, e.textures.diffuse);
            gl::ActiveTexture(gpu::attrs::SPECULAR_TEXTURE_UNIT);
            gl::BindTexture(gl::TEXTURE_2D, e.textures.specular);

            gl::BindVertexArray(e.mesh.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, e.mesh.buffers.indices);
            gl::DrawElements(
                gl::TRIANGLES,
                e.mesh.element_count,
                gl::UNSIGNED_SHORT,
                0 as *const GLvoid
            );

        }
    }

}
