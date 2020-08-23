use super::{IdVal, attrs};
use crate::core::pipeline::mgl;
use mgl::attr::mesh3d::lightmaps::{
    LightMaps,
};
use mgl::s3tc::Image;

use gl::types::*;

#[derive(Debug)]
pub enum Textures {
    Basic(Basic),
    NormalMapped(NormalMapped),
}

#[derive(Default,Debug)]
pub struct Basic {
    // Only id values allowed
    pub diffuse: IdVal,
    pub specular: IdVal,
}

#[derive(Default,Debug)]
pub struct NormalMapped {
    // Only id values allowed
    pub diffuse: IdVal,
    pub specular: IdVal,
    pub normal: IdVal,
}


impl Basic {
    pub fn new() -> Self {
        let mut texs : Self = Default::default();
        unsafe {
            gl::GenTextures((std::mem::size_of::<Self>()/std::mem::size_of::<IdVal>()) as GLsizei,
                           (&mut texs.diffuse) as *mut GLuint);
        }
        texs
    }

    pub fn upload_all_textures(&mut self, lm: &mgl::attr::mesh3d::lightmaps::Basic) {
        upload_s3_texture(&lm.diffuse,  attrs::DIFFUSE_TEXTURE_UNIT, self.diffuse);
        upload_s3_texture(&lm.specular, attrs::SPECULAR_TEXTURE_UNIT, self.specular);
    }
}

impl Drop for Basic {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures((std::mem::size_of::<Self>()/std::mem::size_of::<IdVal>()) as GLsizei,
                           (&mut self.diffuse) as *mut GLuint);
        }
    }
}

impl NormalMapped {
    pub fn new() -> Self {
        let mut texs : Self = Default::default();
        unsafe {
            gl::GenTextures((std::mem::size_of::<Self>()/std::mem::size_of::<IdVal>()) as GLsizei,
                           (&mut texs.diffuse) as *mut GLuint);
        }
        texs
    }

    #[allow(dead_code)]
    pub fn upload_all_textures(&mut self, lm: &mgl::attr::mesh3d::lightmaps::NormalMapped) {
        upload_s3_texture(&lm.diffuse, attrs::DIFFUSE_TEXTURE_UNIT, self.diffuse);
        upload_s3_texture(&lm.specular, attrs::SPECULAR_TEXTURE_UNIT,self.specular);
        upload_s3_texture(&lm.normal,  attrs::NORMAL_TEXTURE_UNIT, self.normal);

        // upload_s3_texture(&lm.diffuse, gl::TEXTURE0, self.diffuse);
        // upload_s3_texture(&lm.specular, gl::TEXTURE1, self.specular);
        // upload_s3_texture(&lm.normal, gl::TEXTURE5, self.normal);
    }
}

impl Drop for NormalMapped {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures((std::mem::size_of::<Self>()/std::mem::size_of::<IdVal>()) as GLsizei,
                           (&mut self.diffuse) as *mut GLuint);
        }
    }
}

#[allow(dead_code)]
impl Textures {
    pub fn new_basic() -> Self {
        Self::Basic(Basic::new())
    }

    pub fn new_normal_mapped() -> Self {
        Self::NormalMapped(NormalMapped::new())
    }
}

impl From<Basic> for Textures {
    fn from(other: Basic) -> Self {
        Textures::Basic(other)
    }
}

impl From<NormalMapped> for Textures {
    fn from(other: NormalMapped) -> Self {
        Textures::NormalMapped(other)
    }
}

fn upload_s3_texture (tex: &Image, tex_unit: GLuint, tex_id: IdVal) {

    // FIXME: Block size is here for a reason
    let _block_size = tex.block_size as i32;
    let format = tex.format.gl_format();

    println!("UPLOADING TEXTURE: {} -> {}", tex_id, tex_unit);

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + tex_unit);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);

        for (level,m) in tex.mipmap_iter().enumerate()  {
            gl::CompressedTexImage2D(gl::TEXTURE_2D, level as i32, format, m.width, m.height,
                                     0, m.data.len() as i32, m.data.as_ptr() as *const GLvoid);
        }

    }
    // Mipmaps are stored by the texture format
    // gl::GenerateMipmap(gl::TEXTURE_2D);
}

impl From<&LightMaps> for Textures {
    fn from (lmaps: &LightMaps) -> Textures {

        let prepared_textures : Textures = match lmaps {
            LightMaps::Basic(lm) => {
                let mut t = Basic::new();
                t.upload_all_textures(&lm);
                t.into()
            },
            LightMaps::NormalMapped(lm) => {
                let mut t = NormalMapped::new();
                t.upload_all_textures(&lm);
                t.into()
            }
        };

        prepared_textures
    }
}
