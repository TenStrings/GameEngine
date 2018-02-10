pub mod math;

#[cfg(test)]
mod tests {
    use super::math::*;
    #[test]
    fn can_add_2d_vectors() {
        let a = Vec2(1, 1);
        let b = Vec2(2, 1);
        assert_eq!(a + b, Vec2(3,2));
    }

    #[test]
    fn can_multiply_2d_vectors_by_scalar() {
        assert_eq!(Vec2(1,1) * (-1), Vec2(-1, -1))
    }
}
