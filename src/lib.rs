pub mod math;
pub mod engine;
pub mod window;
pub mod events;
extern crate num;
extern crate sdl2;
extern crate gl;

pub struct Engine {
    update: Vec<Box<Fn()>>,
    draw: Vec<Box<Fn()>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            draw: Vec::new(),
            update: Vec::new(),
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
