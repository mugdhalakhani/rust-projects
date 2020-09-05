use std::mem;

pub fn run() {
  // length is fixed, elements of the same data type
  println!("Vectors");

  let mut numbers: Vec<i32> = vec![1,2,3,4,5];
  println!("{:?}", numbers);
  println!("{}", numbers[0]);

  println!("{}", numbers[2]);

  // Add to vector
  numbers.push(6);
  numbers.push(7);

  // Pop off the last value
  numbers.pop();

  println!("Array length {}", numbers.len());
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice from an array
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through a vector
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop through a vector
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Numbers: {:?}", numbers);
}