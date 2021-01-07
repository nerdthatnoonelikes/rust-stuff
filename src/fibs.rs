// calculate a set number of fibonnaci numbers from one number
pub fn run() {
    let value1 = 5;
    let value2 = 2;
    let mut counted = 0;
    let mut fibnums: Vec<i32> = vec![0, 1];

    while fibnums[fibnums.len() - 1] != value1 {
        fibnums.push(fibnums[fibnums.len() - 1]+fibnums[fibnums.len() - 2])
    }

    while counted < value2 {
       fibnums.push(fibnums[fibnums.len() - 1] + fibnums[fibnums.len() - 2]);
       counted+=1;
    }
    
    println!("{:?}", fibnums);
}
