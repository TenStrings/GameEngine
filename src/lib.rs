pub mod math;
extern crate num;

#[cfg(test)]
mod tests {
    use super::math::*;
    #[test]
    fn can_add_2d_vectors() {
        let a = Vec2(1.0, 1.0);
        let b = Vec2(2.0, 1.0);
        assert_eq!(a + b, Vec2(3.0,2.0));
    }

    #[test]
    fn can_multiply_2d_vectors_by_scalar() {
        assert_eq!(Vec2(1.0, 1.0) * Scalar(-1.0), Vec2(-1.0, -1.0));
    }

    #[test]
    fn scalar_by_vector() {
        assert_eq!(Vec2(-1.0, -1.0), Scalar(-1.0) * Vec2(1.0, 1.0))
    }

    #[test]
    fn dot_product() {
        assert_eq!(Vec2(1.0,2.0).dot( Vec2(2.0,3.0) ), 2.0 + 2.0*3.0);
    }
}
