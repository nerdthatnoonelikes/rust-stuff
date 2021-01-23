pub fn run(arr: &mut [i32]) {
  let n = arr.len();

  for i in 1..n {
      for j in 0..n-i {
        if arr[j] > arr[j+1] {
            &arr.swap(j, j+1);
        }
      }
  }

  println!("{:?}", arr)
}
