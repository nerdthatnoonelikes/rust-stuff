pub fn run(list: &[i32]) {
   let mut smallest = &list[0];

    for x in list {
        if x < smallest {
            smallest = x
        }
    }

    println!("The smallest number is {}", smallest);
}
