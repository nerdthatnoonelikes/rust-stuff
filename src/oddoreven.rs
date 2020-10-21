pub fn run() {
    let x = 123145435;
    let mut sum = 0;
    for y in x.to_string().bytes() {
        let y = y - b'0';
        sum += y;
      }
      if sum % 2 == 0 {
          println!("even");
      } else {
          println!("odd");
          
      }
}
