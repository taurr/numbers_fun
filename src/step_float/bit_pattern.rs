use core::ops::Deref;
use simple_bitfield::{bitfield, Field};

bitfield! {
    struct Float<u32> {
        mantissa: 23,
        exponent: 8,
        sign: 1
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct StepFloat(f32);

impl Deref for StepFloat {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StepFloat {
    pub fn new(val: f32) -> Self {
        StepFloat(val)
    }

    pub fn increment(self) -> Self {
        let mut value = Float::new(self.to_bits());
        let mantissa = value.mantissa.get();
        let exponent = value.exponent.get();
        if mantissa == 0b11111111111111111111111 {
            value.exponent.set(exponent + 1);
            value.mantissa.set(0);
        } else {
            value.mantissa.set(mantissa + 1);
        };
        Self(f32::from_bits(u32::from(value)))
    }

    pub fn decrement(self) -> Self {
        let mut value = Float::new(self.to_bits());
        let mantissa = value.mantissa.get();
        let exponent = value.exponent.get();
        if mantissa == 0 {
            value.exponent.set(exponent - 1);
            value.mantissa.set(0b11111111111111111111111);
        } else {
            value.mantissa.set(mantissa - 1);
        };
        Self(f32::from_bits(u32::from(value)))
    }
}

#[cfg(test)]
mod test {
    use crate::step_float::StepFloat;
    use proptest::prelude::*;

    #[test]
    fn increments_zero() {
        let sf = StepFloat::new(0.0);
        let sf = sf.increment();
        let value = sf.0.to_bits();
        assert_eq!(1, value);
        let sf = sf.increment();
        let value = sf.0.to_bits();
        assert_eq!(2, value);
    }

    #[test]
    fn increments_one() {
        let sf = StepFloat::new(1.0);
        let one = sf.0.to_bits();
        assert_eq!(one, 0b00111111_10000000_00000000_00000000);
        let sf = sf.increment();
        let value = sf.0.to_bits();
        assert_eq!(value, one + 1);
    }

    #[test]
    fn test_incr_decr_noboundary() {
        let initial = StepFloat::new(0.1);
        let incr = initial.increment();
        let original = incr.decrement();
        assert_eq!(initial, original);
    }

    #[test]
    fn test_incr_decr_boundary() {
        let pattern: u32 = 0b0_00001111_11111111111111111111111;
        let initial = StepFloat::new(f32::from_bits(pattern));
        let original = initial.clone().increment().decrement();
        assert_eq!(initial, original);
    }

    proptest! {
        #[test]
        fn incr_decr_is_noop(p in any::<f32>()) {
            let sf = StepFloat::new(p);
            assert_eq!(sf, StepFloat::new(p).increment().decrement());
        }

        #[test]
        fn incr_decr_n_times_is_noop(p in any::<f32>(), n in 0usize..1024) {
            let mut sf = StepFloat::new(p);
            for _ in 0..n {
                sf = sf.increment();
            }
            for _ in 0..n {
                sf = sf.decrement();
            }
            assert_eq!(sf, StepFloat::new(p));
        }
    }
}
