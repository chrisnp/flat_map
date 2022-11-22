//!
//! Tests for flat_map
//!
//! The tests do not expect any normalization or cleaning.
//! 

use flat_map::*;

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
