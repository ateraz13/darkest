use crate::resource;

#[derive(Debug)]
pub enum InitError {
    SDL2(String),
    GL(String),
    BufferLoader(String)
}

pub struct AppCore {
    pub sdl: sdl2::Sdl,
    pub sdl_video_ctx: sdl2::VideoSubsystem,
    pub sdl_window: sdl2::video::Window,
    pub sdl_event_pump: sdl2::EventPump,
    pub gl_ctx: sdl2::video::GLContext,
    pub buffer_loader: resource::BufferLoader,
}

pub struct AppConfig {
    pub window_size: (u32, u32),
    pub window_title: String,
}

impl AppCore {

    pub fn init(config: AppConfig) -> Result<Self, InitError> {

        macro_rules! unwrap_or_fail {
            ($expr:expr, $fail:expr) => {
                match $expr {
                    Ok(x) => x,
                    Err(e) => return Err($fail(e))
                }
            }
        }

        let sdl = unwrap_or_fail!(sdl2::init(), |e| InitError::SDL2(e));
        let sdl_video = unwrap_or_fail!(sdl.video(), |e| InitError::SDL2(e));

        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);
        gl_attr.set_context_flags()
            .debug().set();

        let window = unwrap_or_fail!(
            sdl_video.window(config.window_title.as_str(),
                             config.window_size.0,
                             config.window_size.1)
                     .opengl()
                     .build(),
            |_| InitError::SDL2("SDL2 Failed constructing window!".to_owned())
        );

        let gl_ctx = unwrap_or_fail!(
            window.gl_create_context(),
            |e| InitError::GL(e)
        );

        let event_pump = unwrap_or_fail!(
            sdl.event_pump(),
            |e| InitError::SDL2(e)
        );

        let _gl = gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let buf_loader = unwrap_or_fail!(
            resource::BufferLoader::relative_to_exe(),
            |e| InitError::BufferLoader(format!("Buffer loader failed init: {:?}", e))
        );

        Ok ( Self {
            sdl: sdl,
            sdl_video_ctx: sdl_video,
            sdl_window: window,
            gl_ctx: gl_ctx,
            sdl_event_pump: event_pump,
            buffer_loader: buf_loader
        })
    }
}
