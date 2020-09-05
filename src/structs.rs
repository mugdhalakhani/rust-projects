// traditional struct
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

// Tuple struct
struct AnotherColor(u8, u8, u8);

// Struct with functions
struct Person {
  first_name: String,
  last_name: String
}

impl Person {
  // ctor
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut color = Color {
    red: 255,
    green: 0,
    blue: 0
  };

  color.red = 200;

  println!("Color: {} {} {}", color.red, color.green, color.blue);

  let mut another_color = AnotherColor(255,0,0);
  another_color.0 = 200;
  println!("Color: {} {} {}", another_color.0, another_color.1, another_color.2);

  let mut person1 = Person::new("Mugdha", "Lakhani");
  println!("person1 {} {}", person1.first_name, person1.last_name);
  println!("person1 {}", person1.full_name());

  person1.set_last_name("Sheth");
  println!("person1 {}", person1.full_name());

  println!("person1 Tuple {:?}", person1.to_tuple());
}