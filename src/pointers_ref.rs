pub fn run() {
   // Primitive array
   let arr1 = [1,2,3];
   let arr2 = arr1;

   println!("Arrays: {:?}", (arr1, arr2));

   // Non primitive: vector
   let vec1 = vec![1,2,3];
   //let vec2 = vec1; // vec1 moved
   let vec2 = &vec1;
   println!("Vectors: {:?}", (&vec1, vec2));

}