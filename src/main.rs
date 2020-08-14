extern crate nalgebra as na;

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


// TODO: Start preparing debug.rs
// OpenGL debugging and error checking modes
//
// Unsafe code should be wrapped with macro
// On failure OpenGL code could be tested in
// debbug mode.

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

    {

        let plane  = helpers::mesh3d::create_plane();

        p3d.activate_shader();

        let light_maps = helpers::mesh3d::load_dds_lightmaps (
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

    let view = {

        let eye_pos = na::Point3::new(0.0, 0.0, 3.0);
        let target = na::Point3::new(0.0, 0.0, 0.0);
        na::Isometry3::look_at_rh(&eye_pos, &target, &na::Vector3::y())

        // let trans = na::Translation3::new(0.0, 0.0, 0.0);
        // let rot = na::UnitQuaternion::from_scaled_axis(na::Vector3::y() * std::f32::consts::FRAC_PI_2);
        // na::Isometry3::from_parts(trans, rot)
    };

    let projection = na::Perspective3::new(4.0 / 3.0, 3.14 / 2.0, 1.0, 1000.0);

    let mut mouse_motion;
    let mut camera_rotate_active = false;
    let mut model_rotation = na::Vector3::new(0.0f32, 0.0f32, 0.0f32);
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
                                model_rotation = na::Vector3::new(0.0f32, 0.0, 0.0);
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


                Event::MouseMotion { mousestate : _mstate, x : _x, y : _y, xrel, yrel, .. } => {
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
        // p3d.activate_shader();

        if camera_rotate_active {
            model_rotation.x += mouse_motion.y;
            model_rotation.y += mouse_motion.x;
        }

        let model =  na::Isometry3::<f32>::new(
            na::Vector3::new(0.0,0.0,-3.0), // translation
            // na::Vector3::new(0.0(time.as_millis() as f32 * 0.0005).sin()-3.14/5.0, 0.0) // rotation
            model_rotation
        );

        let model_mat  = model.to_homogeneous();
        let model_view = view * model;
        let normal_mat = model_view.inverse().to_homogeneous().transpose();
        let proj_mat  = projection.as_matrix();
        let mvp = projection.into_inner() * model_view.to_homogeneous();

        // Drawing code
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Uniform1i(7, 0); // Texture Unit 0 : DIFFUSE
            gl::Uniform1i(8, 1); // Texture Unit 1 : SPECULAR

            p3d.update_projection_matrix(proj_mat.clone());
            p3d.update_view_matrix(view.to_homogeneous());
            p3d.update_model_matrix(0, model_mat);
            p3d.update_normal_matrix(0, normal_mat);

            p3d.draw_textured_meshes();
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
