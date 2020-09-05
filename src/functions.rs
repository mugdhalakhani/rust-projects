pub fn run() {
  greetings("Nice to see you", "Mugdha");
  
  // bind fn val to vars
  let get_sum  = add(5,5);
  println!("Sum: {}", get_sum);

  // Closures
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("Closure sum: {}", add_nums(3,3));
}

fn greetings(greet: &str, name: &str) {
  println!("{}, {}!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2 // no semicolon
}