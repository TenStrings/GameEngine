use sdl2;

pub struct Window {
    pub sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    gl: sdl2::video::GLContext,
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

        Window {
            sdl: sdl,
            window: window,
            gl: gl_context,
        }
    }
}
