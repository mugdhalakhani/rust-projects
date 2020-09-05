pub fn run() {
  println!("conditionals");

  let age = 18;
  let check_id = false;
  let knows_person_of_age = true;

  if age >= 21 && check_id || knows_person_of_age {
    println!("Bartender: What would you like to drink?");
  } else if age <21 && check_id {
    println!("Bartender: Sorry you have to leave");
  } else {
    println!("Bartender: Can I see your ID?");
  }

  // shorthand if
  let is_of_age = if age >= 21 { true } else {false};
  println!("Is of age?: {}", is_of_age);
}