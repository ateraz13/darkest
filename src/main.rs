extern crate cgmath;
#[macro_use]
extern crate cenum;


mod core;
mod resource;
mod helpers;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::io;
use std::path::PathBuf;

use crate::core::pipeline::Pipeline3D;
use crate::core::pipeline::mgl::s3tc;
pub use crate::core::app;

use cgmath::prelude::{ Matrix, SquareMatrix };
use gl::types::*;
use cgmath::prelude::*;


type Matrix4 = cgmath::Matrix4<f32>;
type Vector3 = cgmath::Vector3<f32>;

extern "system" fn gl_error_cb (
    _source : GLenum ,
    err_type : GLenum ,
    _id : GLuint ,
    severity : GLenum ,
    _length : GLsizei ,
    message: *const GLchar,
    _user_param : *mut GLvoid
)
{
    use std::ffi::CStr;
    println!("GL ERROR CALLBACK: {}, type: 0x{}, severity = 0x{}, message = {}",
             if err_type == gl::DEBUG_TYPE_ERROR { "** GL ERROR **"  } else { "" },
             err_type, severity, unsafe {CStr::from_ptr(message).to_str().unwrap()}
    );

}

// TODO: Proposal of new system for managing objects within the scene
// Most of the information about object could be stored in shared pool that
// could be easily accessed by the rendering engine once per cycle.
//
// Additionaly another subsystem could be used to reorganise information changed since last
// frame into groups of commonly accessed values.
//
// For example object visibility can be easily accessed as a boolean value in a vector of bits
// This information could be easily send to shaders with minimal size overhead.
//
// The physics system will be able to organise objects into possibly interactive groups and process
// the currently accessible objects up to date in real time, everything else could be handled in between
// frames if no player can effectivelly be affected by any of the possible physical interactions.
//
//
// TODO: Input handling system
// Supporting independent platforms differently might be pose different problems especially
// if the game is designed with completely different controls in mind. Rebuilding the input system
// would slow down development in the feature and thus I think it would be good idea if flexible system
// is designed now that could be easily manipulated into supporting different input devices.
//
//
// TODO: Primitive physics system could be eather implemented or external physics library could be used.
// Need to learn some opencl or simply use opengl compute shaders to accelerate part of the physics engine.
//
// * Primary target in mind: Nalgebra *
//


fn process_args() -> app::Arguments {

    let mut cmd_args = std::env::args();

    let mut args = app::Arguments {
        game_dir: None,
        print_errors: app::ErrorGroups::NOTHING,
    };

    while let Some(arg) = cmd_args.next() {
        match arg.as_str() {
            "-d" | "--game-dir" => {
                if let Some(path) = cmd_args.next() {
                    let p = PathBuf::from(path);
                    if ! p.exists() {
                        panic!("Path specified for game directory does not exist!");
                    }
                    args.game_dir = Some(p);
                } else {
                    panic!("No path specified for game directory!");
                }
            }
            "--print-gl-errors" => {
                args.print_errors.enable(app::ErrorGroups::GL_ERRORS);
            }
            _ => {
                println!("Unexpected command line arguments: {}", arg);
            }
        }
    }

    args
}

fn main () -> io::Result<()> {

    use std::clone::Clone;

    let app_args = process_args();

    let app_cfg = app::AppConfig {
        args: app_args.clone(),
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

    // Setup debug messaging
    unsafe {

        match_bitfield! {
            match ( app_args.print_errors ) {
                app::ErrorGroups::GL_ERRORS => {
                    gl::Enable(gl::DEBUG_OUTPUT);
                    gl::DebugMessageCallback(gl_error_cb, std::ptr::null());
                }
                app::ErrorGroups::NOTHING => {
                    println!("Error message output is disabled by default!");
                }
            }
        }
    }

    let model_ids = {

        let cube = helpers::mesh3d::load_obj(&app, "./assets/cube.obj").pop().unwrap();
        p3d.activate_shader();

        let susane = helpers::mesh3d::load_obj(&app, "./assets/susane.obj").pop().unwrap();
        p3d.activate_shader();

        let light_maps = helpers::mesh3d::load_dds_normal_mapped_lightmaps (
            &app, "assets/diff.dds", "assets/spec.dds", "assets/norm.dds"
        );

         p3d.prepare_normal_mapped_textured_meshes(&[
            ( &light_maps, &cube ),
            ( &light_maps, &susane )
        ])
    };

    let cube_id = model_ids[0].clone();
    let cube2_id = model_ids[1].clone();

    println!("CUBE_ID.get_type() = {}", cube_id.get_type());

    println!("MODEL_IDS = {:?}", model_ids);

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
                                    unsafe {gl::Uniform1i(core::pipeline::gpu::attrs::USE_BLINN_FLAG, 0);}
                                    enable_blinn = false;
                                } else {
                                    unsafe {gl::Uniform1i(core::pipeline::gpu::attrs::USE_BLINN_FLAG, 1);}
                                    enable_blinn = true;
                                }
                            },

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

        let model_scale = Matrix4::from_scale(
            0.5f32
        );

        let model_mat  = model_scale * cgmath::Matrix4::<f32>::from_translation(
            cgmath::Vector3::new( 1.1, 0.0, 0.0 ),
        );

        let model2_mat  = model_scale * cgmath::Matrix4::<f32>::from_translation(
            cgmath::Vector3::new( -1.1, 0.0, 0.0 ),
        );

        let t = time.as_millis() as f32 / 1000.0;
        let camera_dist = 3.0;
        let camera_pos = cgmath::Point3::<f32>::new(
            camera_dist * t.sin(),
            camera_dist * t.sin() - t.cos(),
            camera_dist *  t.cos()
        );
         
        let view_center = cgmath::Point3::new( 0.0, 0.0, 0.0 );

        // let view_mat = Matrix4::from_translation(
        //     Vector3::new(1.0, 1.0, 5.0)
        // ).invert().unwrap();

        let view_mat = cgmath::Matrix4::<f32>::look_at (
            camera_pos, view_center,
            cgmath::Vector3::new( 0.0, 1.0, 0.0 ),
        );

        // let model_view_mat = view_mat * model_mat;
        // let model_view2_mat = view_mat * model_mat;

        let normal_mat = model_mat.invert().unwrap().transpose();
        let normal2_mat = model2_mat.invert().unwrap().transpose();
        // let normal_mat = model_view_mat.invert().unwrap().transpose();

        let proj_mat  = cgmath::perspective(
            cgmath::Deg(75.0), 4.0/3.0, 0.1, 1000.0
        );

        let _mvp = proj_mat * view_mat * model_mat;

        // Drawing code
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Uniform3fv(10, 1, camera_pos.as_ptr());
        }

        p3d.update_projection_matrix(proj_mat);
        p3d.update_view_matrix(view_mat);

        p3d.update_model_matrix(cube_id, model_mat);
        p3d.update_normal_matrix(cube_id, normal_mat);

        p3d.update_model_matrix(cube2_id, model2_mat);
        p3d.update_normal_matrix(cube2_id, normal2_mat);

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
