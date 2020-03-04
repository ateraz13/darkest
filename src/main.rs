extern crate nalgebra as na;

pub mod resource;
pub mod mgl;
pub mod core;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::io;
use gl::types::*;
use std::fs::File;
use std::io::BufReader;


// TODO: Start preparing debug.rs
// OpenGL debugging and error checking modes
//
// Unsafe code should be wrapped with macro
// On failure OpenGL code could be tested in
// debbug mode.

macro_rules! benchmark {
    {$name:expr; $a:stmt} => {
        let timer = std::time::Instant::now();
        $a
            let time = timer.elapsed();
        println!("benchmark \"{}\": {}ms", $name, time.as_millis());
    }
}

enum S3TC_Format {
    DXT1,
    DXT3,
    DXT5,
}

impl S3TC_Format {

    pub fn gl_format(&self) -> u32 {
        match self {
            DXT1 => gl::COMPRESSED_RGB_S3TC_DXT1_EXT,
            DXT3 => gl::COMPRESSED_RGBA_S3TC_DXT3_EXT,
            DXT5 => gl::COMPRESSED_RGBA_S3TC_DXT5_EXT,
        }
    }

}

struct S3MipmapDesc {
    offset: usize,
    size: usize,
    width: i32,
    height: i32,
}

struct S3MipmapView<'a> {
    width: i32,
    height: i32,
    data: &'a [u8],
}

struct S3Tex {
    pub width: i32,
    pub height: i32,
    pub linear_size: i32,
    pub format: S3TC_Format,
    pub block_size: u32,
    pub data: Vec<u8>,
    pub mipmaps: Vec<S3MipmapDesc>,
}

type MipmapDescIter<'a> = std::slice::Iter<'a, S3MipmapDesc>;

struct S3MipmapIter<'a> {
    data: &'a Vec<u8>,
    desc_iter: MipmapDescIter<'a>
}

impl<'a> S3MipmapIter<'a> {

    pub fn new(data: &'a Vec<u8>, desc_iter: MipmapDescIter<'a>) -> Self {
        Self  {
            data: data,
            desc_iter: desc_iter
        }
    }

}

impl<'a> Iterator for S3MipmapIter<'a> {
    type Item = S3MipmapView<'a>;

    fn next(&mut self) -> Option<S3MipmapView<'a>> {

        match self.desc_iter.next() {
            Some(d) => Some(S3MipmapView {
                width: d.width,
                height: d.height,
                data: &self.data[d.offset .. d.offset+d.size]
            }),
            None => None,
        }

    }
}

#[derive(Debug)]
enum S3TexError {
    UnsupportedCompression,
    InvalidData(String),
}

impl S3Tex {

    fn from_dds_buffer(mut header: Vec<u8>) -> Result<S3Tex, S3TexError> {

        macro_rules! get_u32 {
            ($buffer:ident, $idx:expr) => {
                u32::from_le_bytes([$buffer[$idx], $buffer[$idx+1], $buffer[$idx+2], $buffer[$idx+3]])
            }
        }

        // FIXME: MOVE THIS BACK
        // benchmark! {
        //     "loading_buffer";
        //     let buffer = &app.buffer_loader.load_bytes(Path::new(uri)).unwrap()
        // }
        // TODO:
        // * Check if file ident matches
        // * Generate mipmap descriptors

        let ident = String::from_utf8(header[0..3].to_vec()).unwrap();
        if ident.as_str() != "DDS" {
            return Err(S3TexError::InvalidData("DDS Ident does not much!".to_owned()));
        }

        let buffer = header.drain(128..).collect();

        let width = get_u32!(header, 12);
        let height = get_u32!(header, 16);
        let linear_size = get_u32!(header, 20);
        let mipmap_count = get_u32!(header, 28);
        let four_cc = String::from_utf8(header[84..88].to_vec()).unwrap();

        println!("DXT format: {}", four_cc);
        println!("DXT image size: {},{}", width, height);
        println!("DXT mipmap count: {}", mipmap_count);

        let (format, block_size) = match four_cc.as_str() {
            "DXT1" => ( S3TC_Format::DXT1,  8 ),
            "DXT2" => ( S3TC_Format::DXT3, 16 ),
            "DXT5" => ( S3TC_Format::DXT5, 16 ),
            _ => panic!(format!("Unsupported DXT format! ({})", four_cc))
        };

        let mut mipmaps = vec![];
        let mut mip_w = width;
        let mut mip_h = height;
        let mut mip_offset = 0;

        for _ in 0 .. mipmap_count {

            let mip_size = (((mip_w+3)/4)*((mip_h+3)/4)*block_size) as usize;

            mipmaps.push(
                S3MipmapDesc {
                    offset: mip_offset,
                    size: mip_size,
                    width: mip_w as i32,
                    height: mip_h as i32,
                }
            );

            mip_offset += mip_size;
            mip_w /= 2;
            mip_h /= 2;
        }

        Ok( S3Tex {
            width: width as i32,
            height: height as i32,
            linear_size: linear_size as i32,
            format: format,
            block_size: block_size,
            data: buffer,
            mipmaps: mipmaps
        })
    }

    fn mipmap_iter(&self) ->  S3MipmapIter {
        S3MipmapIter::new(&self.data, self.mipmaps.iter())
    }
}

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

fn main () -> io::Result<()> {

    let app_cfg = core::AppConfig {
        window_size: (800, 600),
        window_title: ("darkest v0.1.0".to_owned())
    };

    let mut app = core::AppCore::init(app_cfg).unwrap();
    configure_texture_parameters();

    let vert_shader = mgl::core::Shader::from_source (
        &app.buffer_loader.load_cstring(Path::new("shaders/basic_vert.glsl"))?,
        gl::VERTEX_SHADER
    ).expect("Could not load vertex shader!`");

    let frag_shader = mgl::core::Shader::from_source (
        &app.buffer_loader.load_cstring(Path::new("shaders/basic_frag.glsl"))?,
        gl::FRAGMENT_SHADER
    ).expect("Could not load fragment shader!");

    let shader_program = mgl::core::ShaderProgram::from_shaders(&[vert_shader, frag_shader])
        .expect("Could not link shader program!");

    let indices: Vec<u16> =  vec![
        0, 1, 3, 2, 3, 1
    ];


    benchmark!{
        "Loading diffuse texture";
        let diffuse_s3_tex = S3Tex::from_dds_buffer(app.buffer_loader.load_bytes(Path::new("assets/diff.dds")).unwrap()).unwrap()
    }

    benchmark!{
        "Loading specular texture";
        let specular_s3_tex = S3Tex::from_dds_buffer(app.buffer_loader.load_bytes(Path::new( "assets/spec.dds" )).unwrap()).unwrap()
    }

    let vert_attrs = mgl::attr::VertexAttributes {

        pos_comp_type: mgl::attr::AttributeType::Vec3,

        // 3 components per position
        positions:  vec![
            -1.0, 1.0, 0.0,  // bottom right
            -1.0, -1.0, 0.0, // bottom left
            1.0, -1.0, 0.0,  // top left
            1.0, 1.0, 0.0,   // top right
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
    };

    let mut position_buffer: GLuint = 0;
    let mut index_buffer: GLuint = 0;
    let mut normal_buffer: GLuint = 0;
    let mut uv_buffer: GLuint = 0;
    let mut diffuse_texture: GLuint = 0;
    let mut specular_texture: GLuint = 0;
    let mut vao: GLuint = 0;

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::Enable(gl::DEPTH_TEST);

        gl::GenBuffers(1, &mut position_buffer);
        gl::GenBuffers(1, &mut normal_buffer);
        gl::GenBuffers(1, &mut uv_buffer);
        gl::GenBuffers(1, &mut index_buffer);
        gl::GenTextures(1, &mut diffuse_texture);
        gl::GenTextures(1, &mut specular_texture);

        gl::BindBuffer(gl::ARRAY_BUFFER, position_buffer);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);

        gl::BufferData(
            gl::ARRAY_BUFFER, // buffer type
            vert_attrs.position_buffer_size(),
            vert_attrs.position_buffer_ptr(),
            gl::STATIC_DRAW // use for the buffer
        );

        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            vert_attrs.normal_buffer_size(),
            vert_attrs.normal_buffer_ptr(),
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, uv_buffer);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            vert_attrs.uv_buffer_size(),
            vert_attrs.uv_buffer_ptr(),
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, position_buffer);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, // location
            3, // component count
            gl::FLOAT, // type
            gl::FALSE, // normalized
            0,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::TRUE,
            0,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(2);
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_buffer);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            0,
            std::ptr::null()
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, position_buffer);
        gl::BindVertexArray(0);


        // FIXME: Upload all the mipmaps to the GPU
        let upload_s3_texture  = |tex: S3Tex, tex_unit: GLenum, tex_id: GLuint| {

            let block_size = tex.block_size as i32;
            let format = tex.format.gl_format();


            gl::ActiveTexture(tex_unit);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            for (level,m) in tex.mipmap_iter().enumerate()  {
                gl::CompressedTexImage2D(gl::TEXTURE_2D, level as i32, format, m.width, m.height,
                                         0, m.data.len() as i32, m.data.as_ptr() as *const GLvoid);
            }
            // gl::GenerateMipmap(gl::TEXTURE_2D);
        };

        upload_s3_texture(diffuse_s3_tex, gl::TEXTURE0, diffuse_texture);
        upload_s3_texture(specular_s3_tex, gl::TEXTURE1, specular_texture);

        // gl::ActiveTexture(gl::TEXTURE0 + 1);
        // gl::BindTexture(gl::TEXTURE_2D, specular_texture);

        // gl::CompressedTexImage2D(gl::TEXTURE_2D, 0, img_diffuse_format, img_specular_w as i32, img_specular_h as i32,
        //                0, gl::UNSIGNED_BYTE as i32, img_specular_pixels[128..].as_ptr() as *const GLvoid);

        // gl::GenerateMipmap(gl::TEXTURE_2D);

    }

    unsafe {
        gl::ClearColor(0.12, 0.0, 0.20, 1.0);
    }


    let timer = std::time::Instant::now();

    let view = {

        let eye_pos = na::Point3::new(0.0, 0.0, 3.0);
        let target = na::Point3::new(0.0, 0.0, 0.0);
        na::Isometry3::look_at_rh(&eye_pos, &target, &na::Vector3::y())

        // let trans = na::Translation3::new(0.0, 0.0, 0.0);
        // let rot = na::UnitQuaternion::from_scaled_axis(na::Vector3::y() * std::f32::consts::FRAC_PI_2);
        // na::Isometry3::from_parts(trans, rot)
    };

    let Vec2 = |x,y| na::Vector2::new(x,y);
    let Vec3 = |x,y,z| na::Vector3::new(x,y,z);

    let projection = na::Perspective3::new(4.0 / 3.0, 3.14 / 2.0, 1.0, 1000.0);

    let mut mouse_motion = Vec2(0.0f32, 0.0f32);
    let mut camera_rotate_active = false;
    let mut model_rotation = Vec3(0.0f32, 0.0f32, 0.0f32);
    let mut enable_blinn = false;

    'main_loop: loop {

        let time = timer.elapsed();

        mouse_motion = na::Vector2::new(0.0f32, 0.0f32);

        for event in app.sdl_event_pump.poll_iter() {

            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode, ..} => {
                    if let Some(k) = keycode {
                        match k {
                            Keycode::Escape => {break 'main_loop;},
                            Keycode::R => {
                                model_rotation = Vec3(0.0f32, 0.0, 0.0);
                            },
                            Keycode::B => {
                                if enable_blinn {
                                    unsafe {gl::Uniform1i(20, 0);}
                                    enable_blinn = false;
                                } else {
                                    unsafe {gl::Uniform1i(20, 1);}
                                    enable_blinn = true;
                                }
                            }
                            _ => {},
                        }
                    }

                }


                Event::MouseMotion { mousestate, x, y, xrel, yrel, .. } => {
                    mouse_motion = na::Vector2::new(xrel as f32 / 800.0 * 3.14,
                                                    yrel as f32 / 600.0 * 3.14);
                },

                Event::MouseButtonDown {mouse_btn, ..} => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            camera_rotate_active = true;
                        }
                        _ => {}
                    }
                },

                Event::MouseButtonUp {mouse_btn, ..} => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            camera_rotate_active = false;
                        }
                        _ => {}
                    }
                },

                _ => {}
            }
        }

        // Enable shader
        shader_program.set_active();

        if camera_rotate_active {
            model_rotation.x += mouse_motion.y;
            model_rotation.y += mouse_motion.x;
        }

        let model =  na::Isometry3::<f32>::new(
            na::Vector3::new(0.0,0.0,0.0), // translation
            // na::Vector3::new(0.0(time.as_millis() as f32 * 0.0005).sin()-3.14/5.0, 0.0) // rotation
            model_rotation
        );

        let model_mat  = model.to_homogeneous(); //na::Matrix4::<f32>::from( rotation );
        let model_view = view * model;
        let normal_mat = model_view.inverse().to_homogeneous().transpose();
        let proj_mat  = projection.as_matrix();
        let mvp = projection.into_inner() * model_view.to_homogeneous();

        // Drawing code
        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::BindVertexArray(vao);

            gl::Uniform1i(7, 0); // Texture Unit 0 : DIFFUSE
            gl::Uniform1i(8, 1); // Texture Unit 0 : SPECULAR

            gl::UniformMatrix4fv(5, 1, gl::FALSE, normal_mat.as_ptr());

            gl::UniformMatrix4fv(
                4, 1, gl::FALSE, mvp.as_ptr()
            );

            gl::UniformMatrix4fv(
                3, 1, gl::FALSE, proj_mat.as_ptr()
            );

            gl::UniformMatrix4fv(
                1, 1, gl::FALSE, model_mat.as_ptr()
            );

            gl::UniformMatrix4fv(
                2, 1, gl::FALSE, (view * model).to_homogeneous().as_ptr(),
            );

            gl::Uniform1f(6, time.as_millis() as f32 / 1000.0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_SHORT,
                0 as *const GLvoid
            )
        }

        app.sdl_window.gl_swap_window();

        // Limit the framerate to 60 FPS
        let time_end = timer.elapsed();
        let frame_duration = time_end - time;

        let max_frame_duration = std::time::Duration::from_millis(1000/60);

        std::thread::sleep(
            max_frame_duration.checked_sub(frame_duration)
                              .unwrap_or(std::time::Duration::new(0,0))
        );
    }

    Ok(())
}
