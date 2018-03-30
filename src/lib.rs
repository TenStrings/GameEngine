#![feature(duration_extras)]
pub mod math;
pub mod engine;
pub mod window;
pub mod events;
extern crate num;
extern crate sdl2;
extern crate gl;

pub struct Engine {
    update: Vec<Box<FnMut(u64)>>,
    draw: Vec<Box<Fn()>>,
    window: window::Window,
    events_pool: events::EventsPool,
}

impl Engine {
    pub fn new() -> Engine {
        let window = window::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title("Test")
        .build();

        window::Window::clear(0.0, 0.0, 0.0, 1.0);

        let events_pool = events::EventsPool::new(&window);
        Engine {
            draw: Vec::new(),
            update: Vec::new(),
            window: window,
            events_pool: events_pool,
        }
    }

    pub fn register_to_update<CB>(&mut self, callback: CB)
        where CB: 'static + FnMut(u64) {
        self.update.push(Box::new(callback));
    }

    pub fn run_game(&mut self) {
        while let Some(_e) = self.events_pool.next() {
            match _e {
                events::Event::Update(dt) =>
                    for cb in self.update.iter_mut() {
                        cb(dt);
                    },
                events::Event::Draw => {
                    for cb in self.draw.iter() {
                        cb();
                    }
                    self.window.swap();
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::math::*;
    use super::engine::*;
    use super::window::*;
    use super::events::*;

    #[test]
    fn can_add_2d_vectors() {
        let a = Vec2(1., 1.);
        let b = Vec2(2., 1.);
        assert_eq!(a + b, Vec2(3., 2.));
    }

    #[test]
    fn can_substract_2d_vectors() {
        let a = Vec2(1., 1.);
        let b = Vec2(1., 2.);
        assert_eq!(a - b, Vec2(0., -1.));
    }

    #[test]
    fn can_multiply_2d_vectors_by_scalar() {
        assert_eq!(Vec2(1., 1.) * Scalar(-1.), Vec2(-1., -1.));
    }

    #[test]
    fn scalar_by_vector() {
        assert_eq!(Vec2(-1., -1.), Scalar(-1.) * Vec2(1., 1.))
    }

    #[test]
    fn dot_product() {
        assert_eq!(Vec2(1.,2.).dot( Vec2(2.,3.) ), Scalar(2. + 2.*3.));
    }

    #[test]
    fn new_world_is_empty() {
        let world = create_world(640, 640);
        assert!(world.particles.is_empty());
    }

    #[test]
    fn can_process_events() {
        let window = WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title("Test")
        .build();

        let mut events_pool = EventsPool::new(&window);
        while let Some(_e) = events_pool.next() {

        }
    }
}
