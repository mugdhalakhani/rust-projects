use std::mem;

pub fn run() {
  // length is fixed, elements of the same data type
  let numbers: [i32; 5] = [1,2,3,4,5];
  println!("{:?}", numbers);
  println!("{}", numbers[0]);

  let mut more_numbers: [i32; 5] = [1,2,3,4,5];
  more_numbers[2] = 20;
  println!("{}", more_numbers[2]);
  println!("Array length {}", more_numbers.len());
  println!("Array occupies {} bytes", mem::size_of_val(&more_numbers));

  // Get slice from an array
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}