extern crate cgmath;

mod core;
mod resource;
mod helpers;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::io;

use crate::core::pipeline::Pipeline3D;
use crate::core::pipeline::mgl;
use crate::core::pipeline::mgl::s3tc;

use cgmath::prelude::{ Matrix, SquareMatrix };
use gl::types::*;

extern "system" fn gl_error_cb( source : GLenum ,
                err_type : GLenum ,
                _id : GLuint ,
                severity : GLenum ,
                _length : GLsizei ,
                message: *const GLchar,
                _user_param : *mut GLvoid)
{
    use std::ffi::CStr;
    println!("GL CALLBACK: 0x{}, type: 0x{}, severity = 0x{}, message = {}",
             if err_type == gl::DEBUG_TYPE_ERROR { "** GL ERROR **"  } else { "" },
             err_type, severity, unsafe {CStr::from_ptr(message).to_str().unwrap()}
    );

}

fn main () -> io::Result<()> {

    pub use crate::core::app;

    let app_cfg = app::AppConfig {
        window_size: (1024, 768),
        window_title: ("darkest v0.1.0".to_owned())
    };

    let mut app = app::AppCore::init(app_cfg).unwrap();

    use crate::core::pipeline::InitError;
    let mut p3d = Pipeline3D::create_and_prepare(&app).map_err(|e| {
        match e {
            InitError::FailedLoadingResource(resource::BufferLoaderError::IoError { file_path, io_error })
                if io_error.kind() == io::ErrorKind::NotFound => {
                    panic!(format!("Resource not found required by core pipeline: {:?}",
                                   file_path.map_or("Path not specified by error".to_owned(), |a| a.display().to_string())))
                },
            InitError::ShaderIssue(issue) => {
                panic!(format!("Core shader issue: {}", issue))
            }
            _ => {}
        }
    }).unwrap();

    unsafe {

        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(gl_error_cb, std::ptr::null());

    }



    {
        let plane  = helpers::mesh3d::create_plane();

        p3d.activate_shader();

        let light_maps = helpers::mesh3d::load_dds_basic_lightmaps (
            &app, "assets/diff.dds", "assets/spec.dds"
        );

        p3d.prepare_textured_meshes(&[
            ( &light_maps, &plane )
        ]);
    }

    unsafe {
        gl::ClearColor(0.12, 0.0, 0.20, 1.0);
    }

    let timer = std::time::Instant::now();

    let mut enable_blinn = false;

    'main_loop: loop {

        let time = timer.elapsed();


        for event in app.sdl_event_pump.poll_iter() {

            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode, ..} => {
                    if let Some(k) = keycode {
                        match k {
                            Keycode::Escape => {break 'main_loop;},
                            Keycode::R => {
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
                },

                Event::MouseMotion { mousestate : _mstate, x : _x, y : _y, xrel : _xrel, yrel : _yrel, .. } => {

                },

                Event::MouseButtonDown {mouse_btn, ..} => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                        }
                        _ => {}
                    }
                },

                Event::MouseButtonUp {mouse_btn, ..} => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                        },
                        _ => {}
                    }
                },

                _ => {}
            }
        }

        let model_mat  = cgmath::Matrix4::<f32>::from_translation(
            cgmath::Vector3::new( 0.0, 0.0, -2.0 ),
        );
        let view_mat = cgmath::Matrix4::<f32>::look_at(
            cgmath::Point3::new( ( time.as_millis() as f32 / 1000.0 ).sin() * -1.0, 0.0, 0.0 ),
            cgmath::Point3::new( 0.0, 0.0, -2.0 ),
            cgmath::Vector3::new( 0.0, 1.0, 0.0 ),
        );
        let model_view_mat = view_mat * model_mat;
        let normal_mat = model_view_mat.invert().unwrap().transpose();
        let proj_mat  = cgmath::perspective(
            cgmath::Deg(90.0), 4.0/3.0, 1.0, 1000.0
        );
        let mvp = proj_mat * view_mat * model_mat;

        // Drawing code
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Uniform1i(7, 0); // Texture Unit 0 : DIFFUSE
            gl::Uniform1i(8, 1); // Texture Unit 1 : SPECULAR
        }

        p3d.update_projection_matrix(proj_mat.clone());
        p3d.update_view_matrix(view_mat.clone());
        p3d.update_model_matrix(0, model_mat);
        p3d.update_normal_matrix(0, normal_mat);

        p3d.draw_textured_meshes();

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
