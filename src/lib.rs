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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flat_map(std::iter::empty(), |x: Vec<()>| {x}).count(), 0);
    }
    
    #[test]
    fn simple() {
        assert_eq!(flat_map(vec!["a", "b"].into_iter(), |x| {x.chars()}).count(), 2);
    }
    
    #[test]
    fn simple_wide() {
        assert_eq!(flat_map(vec!["a1", "bet"].into_iter(), |x| {x.chars()}).count(), 5);
    }
    
    #[test]
    fn from_std_lib_test() {
        let words = ["alpha", "bravo", "charlie"];
        // chars() returns an iterator
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect::<String>();
        assert_eq!(merged, "alphabravocharlie");
    }

    #[test]
    fn empty_middle() {
        let words = ["alpha", "", "bravo", "", "charlie"];
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect::<String>();
        assert_eq!(merged, "alphabravocharlie");
    }

    #[test]
    fn empty_middles_and_outliers() {
        let words = ["", "alpha", "", "", "bravo", "", "", "charlie", ""];
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect::<String>();
        assert_eq!(merged, "alphabravocharlie");
    }

    #[test]
    fn two_dimensional_vector_of_int() {
        let vs: Vec<Vec<i8>> = vec!(vec!(), vec!(0, 1), vec!(), vec!(2, 3, 4, 5), vec!(6), vec!(7, 8, 9), vec!());
        let merged: Vec<_> = flat_map(vs.into_iter(), |ns| ns.into_iter()).collect();
        assert_eq!(merged, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
