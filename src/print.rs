pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // Basic formatting
  println!("{}", 1);
  println!("{} is no {}", "Mugdha", 1);

  // Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

  // Named arguments
  println!("{name} likes to play {activity}", name = "Hanz", activity="soccer");

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10+10);
}