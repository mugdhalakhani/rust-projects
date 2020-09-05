pub fn run() {
  println!("Loops!");

  let mut count = 1;

  // Infinite loop
  // loop {
  //   count += 1;
  //   println!("Number: {}", count);

  //   if count == 20 {
  //     break;
  //   }
  // }

  // while loop fizzbuzz
  while count <= 100 {
    if count % 15 == 0 {
      println!("fizzbuzz");
    } else if count % 3 == 0 {
      println!("fizz");
    } else if count % 5 == 0 {
      println!("buzz");
    } else {
      println!("count {}", count);
    }
    count += 1;
  }

  // for range
  for num in 0..100 {
    if num % 15 == 0 {
      println!("fizzbuzz");
    } else if num % 3 == 0 {
      println!("fizz");
    } else if num % 5 == 0 {
      println!("buzz");
    } else {
      println!("num: {}", num);
    }
  }
}