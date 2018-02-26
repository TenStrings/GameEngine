use math::Vec2;

#[derive(PartialEq)]
pub struct Shape;

#[derive(PartialEq)]
pub struct Particle {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    acceleration: Vec2<f32>,
    shape: Shape,
}

pub struct World {
    pub x: u32,
    pub y: u32,
    pub particles: Vec<Particle>,
}

pub fn create_world(x_dim: u32, y_dim: u32) -> World {
    World {
        x: x_dim,
        y: y_dim,
        particles: Vec::new(),
    }
}
