use sdl2;
use window;
use std::time::{Instant, Duration};


pub enum Event {
    Update(u64),
    Draw,
}

pub struct EventsPool {
    sdl_pump : sdl2::EventPump,
    clock: Instant,
}

impl EventsPool {
    pub fn new(window : &window::Window) -> Self {
        EventsPool {
            sdl_pump : window.sdl.event_pump().unwrap(),
            clock: Instant::now(),
        }
    }

    fn poll_update(&mut self) -> Option<Event> {
        let frame_cap = Duration::from_millis(16);
        let mut elapsed = self.clock.elapsed();
        if elapsed < frame_cap {
            None
        }
        else {
            elapsed = self.clock.elapsed();
            self.clock = Instant::now();
            Some(Event::Update(elapsed.subsec_millis() as u64))
        }
    }
}


impl Iterator for EventsPool {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        //Review the performance
        if let Some(e) = self.sdl_pump.poll_event() {
            match e {
                sdl2::event::Event::Quit { .. } => None,
                _ => Some(Event::Draw),
            }
        }
        else if let Some(e) = self.poll_update() {
            Some(e)
        }
        else {
            //TODO: Add interpolation argument
            Some(Event::Draw)
        }
    }
}
