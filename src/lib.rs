#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "funny_looking_forloops", feature(step_trait))]

pub mod eq_with_tolerance;
#[cfg(feature = "funny_looking_forloops")]
pub mod step_float;
pub mod step_iter;

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
