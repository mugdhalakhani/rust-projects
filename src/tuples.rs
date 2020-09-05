pub fn run() {
 // Max 12 elements in a tuple
 let person: (&str, &str, i8) = ("Mugdha", "London", 33);
 println!("{} is from {} and is {}", person.0, person.1, person.2);
}