/// A 'latching' [`Iterator`].
///
/// The iterator will begin emitting elements as soon as an element satisfies the `start_predicate`
/// and stop *after* an element satisfies the `stop_predicate` in a cycle until the underlying
/// iterator completes.
pub struct LatchIterator<Iter, StartPred, StopPred> {
    inner: Iter,
    start_predicate: StartPred,
    stop_predicate: StopPred,
    latched: bool,
}

impl<Iter: Iterator, StartPred, StopPred> LatchIterator<Iter, StartPred, StopPred>
where
    StartPred: FnMut(&Iter::Item) -> bool,
    StopPred: FnMut(&Iter::Item) -> bool,
{
    /// Create a new [`LatchIterator`]
    ///
    /// It is unusual to invoke this method directly. Instead, the
    /// [`IntoLatchIterator::latch()`](crate::IntoLatchIterator::latch()) function should be used to
    /// convert an iterator into a [`LatchIterator`]
    pub fn new(
        inner: Iter,
        start_predicate: StartPred,
        stop_predicate: StopPred,
    ) -> LatchIterator<Iter, StartPred, StopPred> {
        LatchIterator {
            inner,
            start_predicate,
            stop_predicate,
            latched: false,
        }
    }
}

impl<Iter: Iterator, StartPred, StopPred> Iterator for LatchIterator<Iter, StartPred, StopPred>
where
    StartPred: FnMut(&Iter::Item) -> bool,
    StopPred: FnMut(&Iter::Item) -> bool,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for inner_next in &mut self.inner {
            if self.latched {
                if (self.stop_predicate)(&inner_next) {
                    self.latched = false;
                }
                return Some(inner_next);
            } else if (self.start_predicate)(&inner_next) {
                self.latched = true;
                return Some(inner_next);
            }
        }
        None
    }
}
