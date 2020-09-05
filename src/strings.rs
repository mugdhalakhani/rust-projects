pub fn run() {
 // string literal: "Hello"
 // The other string type is a growable heap allocated data struct.
 let mut hello = String::from("Hello ");
 println!("{}", hello);

 // length
 println!("Length: {}", hello.len());

 hello.push('W');
 println!("{}", hello);
 hello.push_str("orld");
 println!("{}", hello);

 // Capacity in bytes -- no of bytes it can store
 println!("Capacity: {}", hello.capacity());

 println!("Is empty: {}", hello.is_empty());
 println!("Contains world: {}", hello.contains("World"));
 println!("Replaced: {}", hello.replace("World", "there!"));

  // loop through a string by delimiter
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // create a string with capacity in bytes
  let mut string = String::with_capacity(10);
  string.push('r');
  string.push('e');
  string.push('d');
  println!("{}", string);

  assert_eq!(3, string.len());
  assert_eq!(10, string.capacity());
}