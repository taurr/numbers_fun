pub trait EqWithTolerance {
    type N;
    fn eq_with_tolerance(self, other: Self::N, tolerance: Self::N) -> bool;
}

macro_rules! impl_eq_tolerance {
    ($($type:ty)*) => {
        $(
            impl EqWithTolerance for $type {
                type N = $type;
                fn eq_with_tolerance(self, other: Self::N, tolerance: Self::N) -> bool {
                    assert!(tolerance >= Default::default(), "Tolerance must be positive");
                    let diff = self - other;
                    if diff < 0.0 {
                        -diff < tolerance
                    } else {
                        diff < tolerance
                    }
                }
            }
        )*
    };
}

impl_eq_tolerance!(f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn tolerance_must_be_positive() {
        assert!(10f32.eq_with_tolerance(10.49999, -0.5));
    }

    #[test]
    fn within_tolerance_positive() {
        assert!(10f32.eq_with_tolerance(9.500001, 0.5));
        assert!(10f32.eq_with_tolerance(10.49999, 0.5));
    }

    #[test]
    fn within_tolerance_negative() {
        assert!((-10f32).eq_with_tolerance(-9.500001, 0.5));
        assert!((-10f32).eq_with_tolerance(-10.49999, 0.5));
    }

    #[test]
    fn outside_tolerance_positive() {
        assert!(!10f32.eq_with_tolerance(9.5, 0.5));
        assert!(!10f32.eq_with_tolerance(10.5, 0.5));
    }

    #[test]
    fn outside_tolerance_negative() {
        assert!(!(-10f32).eq_with_tolerance(-9.5, 0.5));
        assert!(!(-10f32).eq_with_tolerance(-10.5, 0.5));
    }
}
