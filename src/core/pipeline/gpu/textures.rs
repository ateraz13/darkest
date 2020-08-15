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
            gl::GenBuffers((std::mem::size_of::<Self>()/std::mem::size_of::<IdVal>()) as GLsizei,
                           (&mut texs.diffuse) as *mut GLuint);
        }
        texs
    }
}

impl NormalMapped {
    pub fn new() -> Self {
        let mut texs : Self = Default::default();
        unsafe {
            gl::GenBuffers((std::mem::size_of::<Self>()/std::mem::size_of::<IdVal>()) as GLsizei,
                           (&mut texs.diffuse) as *mut GLuint);
        }
        texs
    }
}

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


impl From<&LightMaps> for Textures {
    fn from (lmaps: &LightMaps) -> Textures {

        let upload_s3_texture  = |tex: &Image, tex_unit: GLenum, tex_id: IdVal| {

            let block_size = tex.block_size as i32;
            let format = tex.format.gl_format();

            unsafe {
                gl::ActiveTexture(tex_unit);
                gl::BindTexture(gl::TEXTURE_2D, tex_id);
                for (level,m) in tex.mipmap_iter().enumerate()  {
                    gl::CompressedTexImage2D(gl::TEXTURE_2D, level as i32, format, m.width, m.height,
                                             0, m.data.len() as i32, m.data.as_ptr() as *const GLvoid);
                }
            }
            // Mipmaps are stored by the texture format
            // gl::GenerateMipmap(gl::TEXTURE_2D);
        };

        let prepared_textures : Textures = match lmaps {
            LightMaps::Basic(lm) => {
                let t = Basic::new();
                upload_s3_texture(&lm.diffuse, attrs::DIFFUSE_TEXTURE_UNIT, t.diffuse);
                upload_s3_texture(&lm.specular, attrs::SPECULAR_TEXTURE_UNIT, t.specular);
                t.into()
            },
            LightMaps::Basic(lm) => {
                let t = NormalMapped::new();
                upload_s3_texture(&lm.diffuse, attrs::DIFFUSE_TEXTURE_UNIT, t.diffuse);
                upload_s3_texture(&lm.specular, attrs::SPECULAR_TEXTURE_UNIT, t.specular);
                upload_s3_texture(&lm.specular, attrs::NORMAL_TEXTURE_UNIT, t.normal);
                t.into()
            }
        };

        prepared_textures
    }
}
