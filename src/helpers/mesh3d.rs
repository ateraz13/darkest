use std::path::Path;

use crate::core::pipeline::mgl;
use crate::core::pipeline::mgl::s3tc;
use crate::core::app;

type Vector3 = cgmath::Vector3<f32>;
type Vector2 = cgmath::Vector2<f32>;

pub fn create_plane() -> mgl::attr::mesh3d::IndexedMesh {
    mgl::attr::mesh3d::IndexedMesh {

            attributes: mgl::attr::mesh3d::VertexAttributes {

                indices: vec![
                    0, 1, 3, 2, 3, 1
                ],

                // 3 components per position
                positions: vec![
                    Vector3::new(-1.0, 1.0, -1.0),  // bottom right
                    Vector3::new(-1.0, -1.0, -1.0), // bottom left
                    Vector3::new(1.0, -1.0, -1.0),  // top left
                    Vector3::new(1.0, 1.0, -1.0),   // top right
                ],

                normals: vec![
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                ],

                uvs: vec! [
                    Vector2::new(1.0, 0.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 1.0)
                ],

                tangents: vec![],
                bitangents: vec![]
        }
    }
}

pub fn create_plane_with_tangents() -> mgl::attr::mesh3d::IndexedMesh {
    let mut im = mgl::attr::mesh3d::IndexedMesh {

            attributes: mgl::attr::mesh3d::VertexAttributes {

                indices: vec![
                    0, 1, 3, 2, 3, 1
                ],

                // 3 components per position
                positions: vec![
                    Vector3::new(-1.0, 1.0, -1.0),  // bottom right
                    Vector3::new(-1.0, -1.0, -1.0), // bottom left
                    Vector3::new(1.0, -1.0, -1.0),  // top left
                    Vector3::new(1.0, 1.0, -1.0),   // top right
                ],

                normals: vec![
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, 1.0),
                ],

                uvs: vec! [
                    Vector2::new(1.0, 0.0),
                    Vector2::new(0.0, 0.0),
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 1.0)
                ],

                tangents: vec![],
                bitangents: vec![]
        }
    };

    im.generate_tangents();
    im
}


pub fn load_dds_basic_lightmaps<P: AsRef<Path>>(app: &app::AppCore, d: P, s: P) -> mgl::attr::mesh3d::lightmaps::Basic {
    mgl::attr::mesh3d::lightmaps::Basic {
            diffuse: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(d.as_ref()).unwrap()).unwrap(),
            specular: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(s.as_ref()).unwrap()).unwrap(),
    }
}

pub fn load_dds_normal_mapped_lightmaps<P: AsRef<Path>>(app: &app::AppCore, diff: P, spec: P, norm: P) -> mgl::attr::mesh3d::lightmaps::NormalMapped {
    mgl::attr::mesh3d::lightmaps::NormalMapped {
            diffuse: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(diff.as_ref()).unwrap()).unwrap(),
            specular: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(spec.as_ref()).unwrap()).unwrap(),
            normal: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(norm.as_ref()).unwrap()).unwrap(),
    }
}
