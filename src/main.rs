#[allow(unused_variables)]
mod oddoreven;
mod sumlist;
mod structs;
mod numberfreq;
mod longest;
mod checktheexam;
mod slices;
mod primes;
mod fibs;
mod generics;
mod bubblesort;
fn main() {
    oddoreven::run();
    sumlist::run();
    structs::run();
    numberfreq::run();
    longest::run();
    checktheexam::run(&["a", "a", "b", "b"], &["a", "c", "b", "d"]);
    slices::run(&[1, 2, 3, 4, 34543, 21, 90809]);
    primes::run();
    fibs::run();
    println!("{}", generics::run(&vec!['a', 'z', 'y', 'c', 'h', 'i']));
    bubblesort::run(&mut vec![64, 34, 25, 12, 22, 11, 90])
}
