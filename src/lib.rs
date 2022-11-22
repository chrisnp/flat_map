pub struct FlatMap<I, F, B> 
where 
    I: Iterator, 
    F: FnMut(I::Item) -> B, 
    B: IntoIterator
{
    iter: I, 
    func: F,
    inner: Option<B::IntoIter>
}

impl<I, F, B> FlatMap<I, F, B> 
where 
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator
{
    fn new(iter: I, func: F) -> Self {
        Self { iter, func, inner: None }
    }
}

impl<I, F, B> Iterator for FlatMap<I, F, B>
where I: Iterator, 
      F: FnMut(I::Item) -> B,
      B: IntoIterator
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner {
                if let Some(val) = inner.next() {
                    return Some(val);
                }
            }
            self.inner = Some((self.func)(self.iter.next()?).into_iter());
        }
    }
}

pub fn flat_map<I, F, B>(iter: I, func: F) -> FlatMap<I, F, B> 
where I: Iterator, 
      F: FnMut(I::Item) -> B,
      B: IntoIterator
{
    FlatMap::new(iter, func)
}
