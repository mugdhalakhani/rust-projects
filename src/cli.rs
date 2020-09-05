use std::env;

pub fn run() {
 let args: Vec<String> = env::args().collect();
 println!("args {:?}", args);

 let command = args[1].clone();
 let name = "Mugdha";
 let status = "100%";

 println!("command {}", command);

 if command == "hello" {
   println!("Hi {}, how are you? ", name);
 } else if command == "status" {
  println!("status is {} ", status);
 } else {
  println!("Invalid command!");
 }

}