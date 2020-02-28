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

    let png_path = std::env::current_exe()?.parent().unwrap().join("assets/container2.png");

    println!("PNG_PATH: {}", png_path.display());
    let ((img_w, img_h), img_pixels) = {
        let img = {
            let generic_img = image::load(
                BufReader::new(File::open(png_path)?),
                image::ImageFormat::Png
            ).unwrap();
            generic_img.as_rgba8().unwrap().clone()
        };


        (img.dimensions(), img.into_raw())
    };

    let vert_attrs = mgl::attr::VertexAttributes {

        pos_comp_type: mgl::attr::AttributeType::Vec3,

        // 3 components per position
        positions:  vec![
            -0.5, 0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0, // bottom left
            0.5, -0.5, 0.0,  // top left
            0.5, 0.5, 0.0,   // top right
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
    let mut texture: GLuint = 0;


    let mut vao: GLuint = 0;


    unsafe {
        gl::Viewport(0, 0, 800, 600);

        gl::GenBuffers(1, &mut position_buffer);
        gl::GenBuffers(1, &mut normal_buffer);
        gl::GenBuffers(1, &mut uv_buffer);
        gl::GenBuffers(1, &mut index_buffer);
        gl::GenTextures(1, &mut texture);

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

        gl::BindTexture(gl::TEXTURE_2D, texture);
        // gl::ActiveTexture(gl::TEXTURE0 + 7);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, img_w as i32, img_h as i32,
                       0, gl::RGBA, gl::UNSIGNED_BYTE, img_pixels.as_ptr() as *const GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    unsafe {
        gl::ClearColor(0.3, 0.0, 0.3, 1.0);
    }


    let timer = std::time::Instant::now();

    let view = {
        let eye    = na::Point3::new(0.0, 0.0, 1.2);
        let target = na::Point3::new(1.0, 0.0, 0.0);
        na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y())
    };

    let projection = na::Perspective3::new(4.0 / 3.0, 3.14 / 2.0, 1.0, 1000.0);

    'main_loop: loop {

        let time = timer.elapsed();

        for event in app.sdl_event_pump.poll_iter() {

            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'main_loop,
                _ => {}
            }
        }

        // Enable shader
        shader_program.set_active();

        let model =  na::Isometry3::<f32>::new(
            na::Vector3::x(), // translation
            na::Vector3::new(0.0, (time.as_millis() as f32 * 0.0005).sin()-3.14/5.0, 0.0) // rotation
        );

        let model_mat  = model.to_homogeneous(); //na::Matrix4::<f32>::from( rotation );
        let model_view = view * model;
        let normal_mat = model_view.inverse().to_homogeneous().transpose();
        let proj_mat  = projection.as_matrix();
        let mvp = projection.into_inner() * model_view.to_homogeneous();

        // Drawing code
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);

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
