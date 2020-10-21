pub fn run() {
    let nums = "1,2,3,4,5".split(",").map(|s| s.parse::<i32>().unwrap());
    let mut sum = 0;
    for x in nums {
        sum += x;
    }
    println!("{}", sum);
}
