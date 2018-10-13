pub struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I, F, B> Iterator for Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(a) => Some((self.f)(a)),
        }
    }
}

pub fn map_lazy<C, F, A, B>(input: C, _function: F) -> Map<C::IntoIter, F>
where
    C: IntoIterator<Item = A>,
    F: FnMut(A) -> B,
{
    Map {
        iter: input.into_iter(),
        f: _function,
    }
}

pub fn map<C, F, A, B>(input: C, _function: F) -> Vec<B>
where
    C: IntoIterator<Item = A>,
    F: FnMut(A) -> B,
{
    map_lazy(input, _function).collect()
}
