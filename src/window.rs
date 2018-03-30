use sdl2;
use std;
use gl;

pub struct Window {
    pub sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    _gl: (),
}

impl Window {
    pub fn clear(r: f32, g: f32, b:f32, a: f32 ) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn swap(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.window.gl_swap_window();
    }
}

pub struct WindowBuilder {
    width: u32,
    heigth: u32,
    title: &'static str,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            width: 640,
            heigth: 640,
            title: "Default",
        }
    }

    pub fn with_title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn with_dimensions(&mut self, width: u32, heigth: u32) -> &mut Self {
        self.width = width;
        self.heigth = heigth;
        self
    }

    pub fn build(&self) -> Window {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(&self.title, self.width, self.heigth)
            .opengl()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        Window {
            sdl: sdl,
            window: window,
            _gl_context: gl_context,
            _gl: gl,
        }
    }
}
