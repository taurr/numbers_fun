#![cfg_attr(feature = "funny_looking_forloops", feature(step_trait))]

use std::ops::Range;

#[cfg(feature = "funny_looking_forloops")]
mod step_float;

// TODO: make featureflag funny_looking_forloops more fine grained
// TODO: cleanup code into modules

// TODO: impl for f32 + f64
pub trait IntoFloatIter<T> {
    type I: IntoIterator<Item = T>;

    /// Iterate over a float range using the local EPSILON
    fn float_iter(self) -> Self::I;
}

// TODO: impl for f32 + f64
pub trait LocalEpsilon {
    /// Return the local EPSILON
    fn epsilon(self) -> Self;
}

pub trait Step<T> {
    type I: IntoIterator<Item = T>;

    fn step(self, step_value: T) -> Self::I;
}

pub struct FloatIterator<T> {
    step_value: T,
    index: usize,
    range: Range<T>,
}

macro_rules! impl_step {
    ([$($type:ty,)*]) => {
        $(
            impl_step!($type);
        )*
    };

    ($type:ty) => {
        impl Step<$type> for Range<$type> {
            type I = FloatIterator<$type>;

            fn step(self, step_value: $type) -> Self::I {
                assert!(step_value > Default::default());
                FloatIterator {
                    step_value,
                    index: 0,
                    range: self,
                }
            }
        }

        impl Iterator for FloatIterator<$type> {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                let current = self.range.start + self.index as $type * self.step_value;
                if current < self.range.end {
                    self.index += 1;
                    Some(current)
                } else {
                    None
                }
            }
        }
    };
}

impl_step!([f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128,]);

pub trait EqTolerance {
    type N;
    fn eq_tolerance(self, other: Self::N, tolerance: Self::N) -> bool;
}

impl EqTolerance for f64 {
    type N = f64;
    fn eq_tolerance(self, other: Self::N, tolerance: Self::N) -> bool {
        // TODO: utilize LocalEpsilon trait
        (self - other).abs() >= tolerance
    }
}

impl EqTolerance for f32 {
    type N = f32;
    fn eq_tolerance(self, other: Self::N, tolerance: Self::N) -> bool {
        // TODO: utilize LocalEpsilon trait
        (self - other).abs() < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn there_can_be_issues_with_float() {
        let mut prev = None;
        for x in (20_000_000f32..20_000_002f32).step(0.1) {
            if let Some(prev) = prev {
                assert!(prev < x);
            }
            prev = Some(x);
        }
    }

    #[test]
    fn it_works() {
        let steps = (0f32..1f32).step(0.1).collect::<Vec<f32>>();
        for (a, b) in [0f32, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
            .into_iter()
            .zip(steps)
        {
            assert!(a.eq_tolerance(b, f32::EPSILON));
        }

        // TODO: are these equal after optimizer?
        let _ = (0i32..100).step(10).collect::<Vec<i32>>();
        let _ = (0i32..100).into_iter().skip(10).collect::<Vec<i32>>();
    }
}
