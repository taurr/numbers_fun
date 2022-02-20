use core::ops::Range;

pub trait IntoIterStep<T> {
    type I: IntoIterator<Item = T>;
    fn into_iter_step(self, step: T) -> Self::I;
}

pub struct StepIterator<T> {
    step: T,
    count: usize,
    range: Range<T>,
}

macro_rules! impl_step {
    ($($type:ty)*) => {
        $(
            impl IntoIterStep<$type> for Range<$type> {
                type I = StepIterator<$type>;

                fn into_iter_step(self, step: $type) -> Self::I {
                    assert!(step > Default::default());
                    StepIterator {
                        step,
                        count: 0,
                        range: self,
                    }
                }
            }

            impl Iterator for StepIterator<$type> {
                type Item = $type;

                fn next(&mut self) -> Option<Self::Item> {
                    let current = self.range.start + self.count as $type * self.step;
                    if current < self.range.end {
                        self.count += 1;
                        Some(current)
                    } else {
                        None
                    }
                }
            }
        )*
    };
}
impl_step!(f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eq_with_tolerance::EqWithTolerance;

    #[test]
    #[should_panic]
    fn there_can_be_issues_with_float() {
        let mut prev = None;
        for x in (20_000_000f32..20_000_002f32).into_iter_step(0.1) {
            if let Some(prev) = prev {
                assert!(prev < x);
            }
            prev = Some(x);
        }
    }

    #[test]
    fn issues_can_be_overcome() {
        let numbers = (20_000_000f32..20_000_002f32)
            .into_iter_step(0.1)
            .collect::<Vec<_>>();
        assert_eq!(11, dbg!(numbers).len());
    }

    #[test]
    fn it_works() {
        let steps = (0f32..1f32).into_iter_step(0.1).collect::<Vec<f32>>();
        for (a, b) in [0f32, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
            .into_iter()
            .zip(steps)
        {
            assert!(a.eq_with_tolerance(b, f32::EPSILON));
        }

        // TODO: are these equal after optimizer?
        let _ = (0i32..100).into_iter_step(10).collect::<Vec<i32>>();
        let _ = (0i32..100).into_iter().skip(10).collect::<Vec<i32>>();
    }
}
