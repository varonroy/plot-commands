pub trait MapReduce {
    type Item;

    fn map_reduce<F0, F, T>(self, f0: F0, f: F) -> Option<T>
    where
        Self: Sized,
        F0: FnOnce(Self::Item) -> T,
        F: FnMut(T, Self::Item) -> T;
}

impl<I, IT: Iterator<Item = I>> MapReduce for IT {
    type Item = I;

    fn map_reduce<F0, F, T>(mut self, f0: F0, f: F) -> Option<T>
    where
        Self: Sized,
        F0: FnOnce(I) -> T,
        F: FnMut(T, Self::Item) -> T,
    {
        let first = f0(self.next()?);
        Some(self.fold(first, f))
    }
}
