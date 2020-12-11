#[allow(unused_variables)]
mod oddoreven;
mod sumlist;
mod structs;
mod numberfreq;
mod longest;
mod checktheexam;
fn main() {
    oddoreven::run();
    sumlist::run();
    structs::run();
    numberfreq::run();
    longest::run();
    checktheexam::run(&["a", "a", "b", "b"], &["a", "c", "b", "d"]);
}
