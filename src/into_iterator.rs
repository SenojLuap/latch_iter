use crate::iterator::LatchIterator;

/// Trait that indicates ability to generate a [`LatchIterator`]
pub trait IntoLatchIterator<Iter: Iterator> {
    /// Consume the [`Iterator`] and wrap it in a [`LatchIterator`]
    ///
    /// The iterator will begin emitting elements as soon as an element satisfies the `start_predicate`
    /// and stop *after* an element satisfies the `stop_predicate` in a cycle until the underlying
    /// iterator completes.
    ///
    /// # Examples
    ///
    /// ```
    /// /// Begins emitting on multiples of 5, and stops on multiples of 7
    /// for number in (0_u32..25_u32).latch(|&num| num % 5 == 0, |&num| num % 7 == 0) {
    ///   println!("Next: {}", number);
    /// }
    /// ```
    fn latch<StartPred, StopPred>(
        self,
        start_predicate: StartPred,
        stop_predicate: StopPred,
    ) -> LatchIterator<Iter, StartPred, StopPred>
    where
        StartPred: FnMut(&Iter::Item) -> bool,
        StopPred: FnMut(&Iter::Item) -> bool;
}
