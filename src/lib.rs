#[cfg(test)]
mod tests {
    // use super::*;

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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
