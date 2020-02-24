pub mod resource;
pub mod mgl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ffi::{CString, CStr};
use std::path::Path;
use std::io;
use gl::types::*;


// TODO: Start preparing debug.rs
// OpenGL debugging and error checking modes
//
// Unsafe code should be wrapped with macro
// On failure OpenGL code could be tested in
// debbug mode.


pub enum AttributeType {
    Vec2, Vec3, Vec4
}


pub struct VertexAttributes {
    pub pos_comp_type: AttributeType,
    // NOTE: we do not use vector types for attributes because we may want
    // different number of components for some attributes
    pub positions: Vec<f32>, // 2 or 3 components per position
    pub normals: Vec<f32>, // 3 components per normal
    pub uvs: Vec<f32> // 2 components per uv
}

impl VertexAttributes {

    pub fn position_buffer_size(&self) -> GLsizeiptr {
        (self.positions.len() * std::mem::size_of::<f32>()) as GLsizeiptr
    }

    pub fn normal_buffer_size(&self) -> GLsizeiptr {
        (self.normals.len() * std::mem::size_of::<f32>()) as GLsizeiptr
    }

    pub fn uv_buffer_size(&self) -> GLsizeiptr {
        (self.uvs.len() * std::mem::size_of::<f32>()) as GLsizeiptr
    }

    pub unsafe fn position_buffer_ptr(&self) -> *const GLvoid {
        unsafe {
            self.positions.as_ptr() as *const GLvoid
        }
    }

    pub unsafe fn normal_buffer_ptr(&self) -> *const GLvoid {
        unsafe {
            self.normals.as_ptr() as *const GLvoid
        }
    }

    pub unsafe fn uv_buffer_ptr(&self) -> *const GLvoid {
        unsafe {
            self.uvs.as_ptr() as *const GLvoid
        }
    }
}

fn main()  -> io::Result<()>{

    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video()
                       .expect("Could not initialise SDL2 context!");

    let gl_attr = sdl_video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = sdl_video
        .window("darkest v0.01a", 800, 600)
        .opengl()
//        .resizable()
        .build()
        .expect("Could not create SDL2 window!");

    let gl_ctx = window.gl_create_context()
                       .expect("Could not initialise OpenGL context!");

    let gl = gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump()
                            .expect("Could not initialise SDL2 event pump!");

    let buf_loader = resource::BufferLoader::relative_to_exe()?;

    let vert_shader = mgl::core::Shader::from_source (
        &buf_loader.load_cstring(Path::new("shaders/basic_vert.glsl"))?,
        gl::VERTEX_SHADER
    ).expect("Could not load vertex shader!`");

    let frag_shader = mgl::core::Shader::from_source (
        &buf_loader.load_cstring(Path::new("shaders/basic_frag.glsl"))?,
        gl::FRAGMENT_SHADER
    ).expect("Could not load fragment shader!");

    let shader_program = mgl::core::ShaderProgram::from_shaders(&[vert_shader, frag_shader])
        .expect("Could not link shader program!");

    let indices: Vec<u16> =  vec![
        0, 1, 3, 2, 3, 1
    ];

    let vert_attrs = VertexAttributes {

        pos_comp_type: AttributeType::Vec3,

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
            1.0, 0.0,
            1.0, 1.0
        ]
    };

    let mut position_buffer: GLuint = 0;
    let mut index_buffer: GLuint = 0;
    let mut normal_buffer: GLuint = 0;
    let mut uv_buffer: GLuint = 0;

    let mut vao: GLuint = 0;

    unsafe {
        gl::Viewport(0, 0, 800, 600);

        gl::GenBuffers(1, &mut position_buffer);
        gl::GenBuffers(1, &mut normal_buffer);
        gl::GenBuffers(1, &mut uv_buffer);
        gl::GenBuffers(1, &mut index_buffer);

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
    }

    unsafe {
        gl::ClearColor(0.3, 0.0, 0.3, 1.0);
    }

    'main_loop: loop {

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'main_loop,
                _ => {}
            }
        }

        // Enable shader
        shader_program.set_active();

        // Drawing code
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            // gl::DrawArrays(
            //     gl::TRIANGLES, // mode
            //     0, // start index
            //     3, // number of indices
            // );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_SHORT,
                0 as *const GLvoid
            )
        }

       window.gl_swap_window();
    }

    Ok(())
}
