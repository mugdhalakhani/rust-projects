pub fn run() {
  let name = "Mugdha";
  let mut age = 33;
  println!("My name is {} and I am {}", name, age);
  age = 34;
  println!("My name is {} and I am {}", name, age);

  // Define a constant of type 32 bit int.
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let ( my_name, my_age) = ("Mugdha", 36);
  println!("{} is {}", my_name, my_age);
}