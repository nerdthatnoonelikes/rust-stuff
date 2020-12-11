// Original problem link: https://www.codewars.com/kata/5a3dd29055519e23ec000074/

pub fn run(arr_a: &[&str], arr_b: &[&str]) {
    let mut score = 0; 
    let mut current = 0;
    while current < arr_b.len() {
        if arr_a[current] == arr_b[current] {
            current += 1;
            score += 4;
        } else if arr_b[current] == "" {
            current += 1;
        } else {
            current += 1;
            score -= 1;
        }
    }
    println!("Exam score is: {}", score);
}