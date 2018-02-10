use std::ops::{Add, Mul};

#[derive(Debug, PartialEq)]
pub struct Vec2<T>(pub T,pub T);

impl<T: Add<Output=T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, other: Self) -> Vec2<T> {
        Vec2(
            self.0 + other.0,
            self.1 + other.1,
        )
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Vec2<T> {
        Vec2(
            self.0 * scalar,
            self.1 * scalar,
        )
    }


}
