
pub fn run() {
  let x =1; // default is i32
  let y = 2.5; // inferred to a f64
  
  // Add explicit type
  let z: i64 = 444545234;

   // find max size
   println!("Max i32: {}", std::i32::MAX);
   println!("Max i64: {}", std::i64::MAX);

   // Boolean
   let is_active = true;

   // Get bool from expression
   let is_greater =  10 < 5;

   // Char, can include emojis
   let a1 = 'b';
   let face = '\u{1F600}';

   println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

}