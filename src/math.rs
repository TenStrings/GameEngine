use std::ops::{Add, Mul, Sub, Div};
use num::Float;

#[derive(Debug, PartialEq)]
pub struct Vec2<T: Float>(pub T,pub T);

#[derive(Debug, PartialEq)]
pub struct Scalar<T: Float>(pub T);

impl<T: Float> Vec2<T>{

    pub fn dot(self, other: Self) -> Scalar<T> {
        Scalar(self.0 * other.0 + self.1 * other.1)
    }
}

impl<T: Float> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, other: Self) -> Vec2<T> {
        Vec2(
            self.0 + other.0,
            self.1 + other.1,
        )
    }
}

impl<T: Float> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, other: Self) -> Vec2<T> {
        Vec2(
            self.0 - other.0,
            self.1 - other.1,
        )
    }
}

impl<T: Float> Mul<Scalar<T>> for Vec2<T> {
    type Output = Self;

    fn mul(self, scalar: Scalar<T>) -> Vec2<T> {
        let Scalar(s) = scalar;
        Vec2(
            self.0 * s,
            self.1 * s,
        )
    }
}

impl<T: Float> Mul<Vec2<T>> for Scalar<T> {
    type Output = Vec2<T>;

    fn mul(self, vec: Vec2<T>) -> Vec2<T> {
        Vec2(
            vec.0 * self.0,
            vec.1 * self.0,
        )
    }
}

//
//Traits for Scalar `f` Scalar
//TODO: Make this things a macro (and add others...)
//

impl<T: Float> Add<Scalar<T>> for Scalar<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Scalar(self.0 + rhs.0)
    }
}

impl<T: Float> Sub<Scalar<T>> for Scalar<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Scalar(self.0 - rhs.0)
    }
}

impl<T: Float> Mul<Scalar<T>> for Scalar<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Scalar(self.0 * rhs.0)
    }
}

impl<T: Float> Div<Scalar<T>> for Scalar<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Scalar(self.0 / rhs.0)
    }
}
