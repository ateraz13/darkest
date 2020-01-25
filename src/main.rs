extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ffi::{CString, CStr};

struct Shader {
    id: gl::types::GLuint,
}

impl Shader {

}

fn prepare_whitespaced_cstring_with_len(len: usize) -> CString {

    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '.iter().cycle().take(len)]);
    unsafe { CString::from_vec_unchecked(buffer) }
}

impl Shader {

    pub fn from_source (
        shader_source: &CStr,
        shader_type: gl::types::GLenum
    ) -> Result<Self, String> {

        use gl::types::*;

        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::Shaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 1;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mesg = prepare_whitespaced_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id, len,
                    std::ptr::null_mut(),
                    mesg.as_ptr() as *mut GLchar
                );
            }

            Err(error.to_string_lossy().into_owned());
        }

        Ok(Self { id })
    }

    fn main() {

        let sdl = sdl2::init().unwrap();
        let sdl_video = sdl.video()
                           .expect("Could not initialise SDL2 context!");

        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = sdl_video
            .window("darkest v0.01a", 800, 600)
            .opengl()
            .resizable()
            .build()
            .expect("Could not create SDL2 window!");

        let gl_ctx = window.gl_create_context()
                           .expect("Could not initialise OpenGL context!");

        let gl = gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let mut event_pump = sdl.event_pump()
                                .expect("Could not initialise SDL2 event pump!");

        unsafe {
            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.3, 0.0, 0.3, 1.0);
        }

        'main_loop: loop {
            for event in event_pump.poll_iter() {

                match event {
                    Event::Quit {..} => break 'main_loop,
                    Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'main_loop,
                    _ => {},

                }
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            window.gl_swap_window();
        }


    }

}
