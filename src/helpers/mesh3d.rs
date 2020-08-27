use std::path::Path;
use crate::core::pipeline::mgl::attr::mesh3d;

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


pub fn load_dds_basic_lightmaps<P: AsRef<Path>>(app: &app::AppCore, d: P, s: P) -> mesh3d::lightmaps::Basic {
    mgl::attr::mesh3d::lightmaps::Basic {
            diffuse: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(d.as_ref()).unwrap()).unwrap(),
            specular: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(s.as_ref()).unwrap()).unwrap(),
    }
}

pub fn load_dds_normal_mapped_lightmaps<P: AsRef<Path>>(app: &app::AppCore, diff: P, spec: P, norm: P) -> mesh3d::lightmaps::NormalMapped {
    mgl::attr::mesh3d::lightmaps::NormalMapped {
            diffuse: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(diff.as_ref()).unwrap()).unwrap(),
            specular: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(spec.as_ref()).unwrap()).unwrap(),
            normal: s3tc::Image::from_dds_buffer(app.buffer_loader.load_bytes(norm.as_ref()).unwrap()).unwrap(),
    }
}


struct MakeVector3Iter<'a, I: Iterator<Item = &'a f32>>{
    iter: I,
}

impl<'a, I: Iterator<Item = &'a f32>> MakeVector3Iter<'a, I> {

    fn from(i: I) -> Self  {
        Self {
            iter: i
        }
    }

}

impl<'a, I: Iterator<Item = &'a f32>> Iterator for MakeVector3Iter<'a, I> {
    type Item = Vector3;

    fn next(&mut self) -> Option<Vector3> {
        Some( Vector3::new(
            *self.iter.next()?,
            *self.iter.next()?,
            *self.iter.next()?
        ))
    }
}

struct MakeVector2Iter<'a, I: Iterator<Item = &'a f32>>{
    iter: I,
}

impl<'a, I: Iterator<Item = &'a f32>> MakeVector2Iter<'a, I> {

    fn from(i: I) -> Self  {
        Self {
            iter: i
        }
    }

}

impl<'a, I: Iterator<Item = &'a f32>> Iterator for MakeVector2Iter<'a, I> {
    type Item = Vector2;

    fn next(&mut self) -> Option<Vector2> {
        Some( Vector2::new(
            *self.iter.next()?,
            *self.iter.next()?,
        ))
    }
}


pub fn load_obj<P: AsRef<Path>>( p: P ) -> Vec<mesh3d::IndexedMesh> {

    use std::io;
    use std::fs;

    // let f = fs::File::open(p.as_ref()).unwrap();
    // // let buf = io::BufReader::new(&f);

    let (models, materials) = tobj::load_obj(p.as_ref(), true).unwrap();

    models.iter().enumerate().map(|(i, model)| {
        let mesh = &model.mesh;

        let mut im = mesh3d::IndexedMesh {
            attributes: mesh3d::VertexAttributes {
                indices: mesh.indices.clone(),
                positions: MakeVector3Iter::from(mesh.positions.iter()).collect(),
                normals: MakeVector3Iter::from(mesh.normals.iter()).collect(),
                uvs: MakeVector2Iter::from(mesh.texcoords.iter())
                    .map(|v| Vector2::new(v.x, 1.0-v.y))
                    .collect(),
                tangents: vec![],
                bitangents: vec![],
            }
        };

        im.generate_tangents();
        im

    }).collect()
}
