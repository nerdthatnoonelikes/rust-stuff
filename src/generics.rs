use std::fmt::Display;
pub fn run<T: Display + PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for x in list {
        if x > largest {
            largest = x;
        }
    }

    largest
}