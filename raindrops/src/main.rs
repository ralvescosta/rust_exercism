fn main() {
  let vec = [1,2,3,4];

  // for i in 0..vec.len() {
  //   println!("{}", vec[i]);
  // }

  for element in vec.iter() {
    println!("{}", element);
  }
}