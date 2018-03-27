use sdl2;
use window;

pub enum Event {
    Quit,
}

pub struct EventsPool {
    sdl_pump : sdl2::EventPump,
}

impl EventsPool {
    pub fn new(window : &window::Window) -> Self {
        EventsPool {
            sdl_pump : window.sdl.event_pump().unwrap(),
        }
    }
}

impl Iterator for EventsPool {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        match self.sdl_pump.poll_event().unwrap() {
            sdl2::event::Event::Quit { .. } => None,
            _ => None,
        }
    }
}
