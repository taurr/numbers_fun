pub use bit_pattern::StepFloat;
pub use core::iter::Step;

pub mod bit_pattern;

impl Step for StepFloat {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        // TODO this is stupid way of doing it, can math help?
        let mut start = *start;
        let mut counter = 0;
        while start != *end {
            start = start.increment();
            counter += 1;
        }
        Some(counter)
    }

    fn forward_checked(mut start: Self, count: usize) -> Option<Self> {
        for _ in 0..count {
            start = start.increment();
        }
        Some(start)
    }

    fn backward_checked(mut start: Self, count: usize) -> Option<Self> {
        for _ in 0..count {
            start = start.decrement();
        }
        Some(start)
    }
}

#[cfg(test)]
mod test {
    use super::StepFloat;

    #[test]
    fn float_loop() {
        for step in StepFloat::new(200000.0)..StepFloat::new(200001.0) {
            println!("{:x}", step.to_bits());
        }
    }
}
