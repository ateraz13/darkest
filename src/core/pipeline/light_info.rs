type Vec3 = cgmath::Vector3<f32>;

#[allow(dead_code)]
pub struct DirLight {
    intensity: f32,
    direction: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

#[allow(dead_code)]
struct PointLight {
    position: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

#[allow(dead_code)]
impl DirLight {
    pub fn default() -> Self {
        Self {
            intensity: 0.5,
            direction: Vec3::new(0.0, -1.0, 1.0),
            ambient: Vec3::new(0.5, 0.5, 0.5),
            diffuse: Vec3::new(0.5, 0.5, 0.5),
            specular: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}

#[allow(dead_code)]
impl PointLight {
    pub fn default() -> Self {
        Self {
            position: Vec3::new(1.0, 1.0, 1.0),
            ambient: Vec3::new(0.5, 0.5, 0.5),
            diffuse: Vec3::new(0.5, 0.5, 0.5),
            specular: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}
