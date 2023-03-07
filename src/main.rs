use std::io;

fn main() {
  println!("Enter your weight on earth (in kg): ");

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  
  let weight_on_earth: f32 = input.trim().parse().unwrap();
  println!("Your weight on mars will be {}kg", calculate_weight_on_mars(weight_on_earth));
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
  weight_on_earth / 9.81 * 3.711
}
