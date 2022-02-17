use core::ops::Deref;

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
        let value = self.0.to_bits();
        let mantissa = value & 0b0_00000000_11111111111111111111111;
        let incr = if mantissa == 0b0_00000000_11111111111111111111111 {
            let mut exponent = value & 0b0_11111111_00000000000000000000000;
            exponent >>= 23;
            exponent += 1;
            exponent <<= 23;
            let mut value = value & 0b1_00000000_11111111111111111111111;
            value |= exponent;
            // reset mantissa
            value & 0b1_11111111_00000000000000000000000
        } else {
            let mut mantissa = value & 0b0_00000000_11111111111111111111111;
            mantissa += 1;
            let value = value & 0b1_11111111_00000000000000000000000;
            value | mantissa
        };
        Self(f32::from_bits(incr))
    }

    pub fn decrement(self) -> Self {
        let value = self.0.to_bits();
        let mantissa = value & 0b0_00000000_11111111111111111111111;
        let decr = if mantissa == 0b0_00000000_00000000000000000000000 {
            let mut exponent = value & 0b0_11111111_00000000000000000000000;
            exponent >>= 23;
            exponent -= 1;
            exponent <<= 23;
            let value = value & 0b1_00000000_11111111111111111111111;
            let value = value | exponent;
            // set mantissa
            value | 0b0_00000000_11111111111111111111111
        } else {
            let mut mantissa = value & 0b0_00000000_11111111111111111111111;
            mantissa -= 1;
            let value = value & 0b1_11111111_00000000000000000000000;
            value | mantissa
        };
        Self(f32::from_bits(decr))
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
        let value = unsafe { std::mem::transmute::<f32, u32>(sf.0) };
        assert_eq!(1, value);
        let sf = sf.increment();
        let value = unsafe { std::mem::transmute::<f32, u32>(sf.0) };
        assert_eq!(2, value);
    }

    #[test]
    fn increments_one() {
        let sf = StepFloat::new(1.0);
        let one = unsafe { std::mem::transmute::<f32, u32>(sf.0) };
        assert_eq!(one, 0b00111111_10000000_00000000_00000000);
        let sf = sf.increment();
        let value = unsafe { std::mem::transmute::<f32, u32>(sf.0) };
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
        let initial = StepFloat::new(unsafe { std::mem::transmute(pattern) });
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
