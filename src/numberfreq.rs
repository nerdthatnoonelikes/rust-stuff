use std::collections::HashMap;
pub fn run() {
    let nums = "08989082882348823838";
    let mut map: HashMap<char, i32> = HashMap::new();

    for num in nums.chars() {
        let insert = map.entry(num).or_insert(0);
        *insert += 1;
    }
    println!("Number that appears the most times is {:?}", map.iter().max_by_key(|(_, &value)| value));
}