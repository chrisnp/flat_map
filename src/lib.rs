#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn empty() {
        assert_eq!(flat_map(std::iter::empty(), |x: Vec<()>| {x}).count(), 0);
    }
    
    #[test]
    #[ignore]
    fn simple() {
        assert_eq!(flat_map(vec!["a", "b"].into_iter(), |x| {x.chars()}).count(), 2);
    }
    
    #[test]
    #[ignore]
    fn simple_wide() {
        assert_eq!(flat_map(vec!["a1", "bet"].into_iter(), |x| {x.chars()}).count(), 5);
    }
    
    #[test]
    #[ignore]
    fn from_std_lib_test() {
        let words = ["alpha", "bravo", "charlie"];
        // chars() returns an iterator
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect::<String>();
        assert_eq!(merged, "alphabravocharlie");
    }

    #[test]
    #[ignore]
    fn empty_middle() {
        let words = ["alpha", "", "bravo", "", "charlie"];
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect::<String>();
        assert_eq!(merged, "alphabravocharlie");
    }

    #[test]
    #[ignore]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
