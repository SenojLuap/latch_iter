//! Provies the [`LatchIterator`] via the [`IntoLatchIterator::latch()`] function

mod into_iterator;
mod iterator;

pub use into_iterator::IntoLatchIterator;
pub use iterator::LatchIterator;

impl<Iter: Iterator> IntoLatchIterator<Iter> for Iter {
    fn latch<StartPred, StopPred>(
        self,
        start_predicate: StartPred,
        stop_predicate: StopPred,
    ) -> LatchIterator<Iter, StartPred, StopPred>
    where
        StartPred: FnMut(&Iter::Item) -> bool,
        StopPred: FnMut(&Iter::Item) -> bool,
    {
        LatchIterator::new(self, start_predicate, stop_predicate)
    }
}

#[cfg(test)]
mod tests {
    use crate::IntoLatchIterator;

    #[test]
    fn range_test() {
        let results = (1_u32..25_u32)
            .latch(|&num| num % 5 == 0, |&num| num % 7 == 0)
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec!(5, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21)
        );
    }
}
