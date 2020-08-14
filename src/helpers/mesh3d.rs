
use std::path::Path;

use crate::core::pipeline::mgl;
use crate::core::pipeline::mgl::s3tc;
use crate::core::app;

pub fn create_plane() -> mgl::attr::mesh3d::IndexedMesh {
    mgl::attr::mesh3d::IndexedMesh {

            indices: vec![
                0, 1, 3, 2, 3, 1
            ],

            attributes: mgl::attr::mesh3d::VertexAttributes {

                pos_comp_type: mgl::attr::AttributeType::Vec3,

                // 3 components per position
                positions: vec![
                    -1.0, 1.0, -1.0,  // bottom right
                    -1.0, -1.0, -1.0, // bottom left
                    1.0, -1.0, -1.0,  // top left
                    1.0, 1.0, -1.0,   // top right
                ],

                normals: vec![
                    0.0, 0.0, 1.0,
                    0.0, 0.0, 1.0,
                    0.0, 0.0, 1.0,
                    0.0, 0.0, 1.0,
                ],

                uvs: vec! [
                    1.0, 0.0,
                    0.0, 0.0,
                    0.0, 1.0,
                    1.0, 1.0
                ]
            }
        }
}


pub fn load_dds_lightmaps<P: AsRef<Path>>(app: &app::AppCore, d: P, s: P) -> mgl::attr::mesh3d::LightMaps {
    mgl::attr::mesh3d::LightMaps {
            diffuse: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(d.as_ref()).unwrap()).unwrap(),
            specular: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(s.as_ref()).unwrap()).unwrap(),
    }
}
