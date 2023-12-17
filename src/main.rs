use std::io;
use std::io::Write;

mod basic;

fn main() {
  println!("Hello Math app world!");

  // get input from user
  let mut input = String::new();

  print!("Input: ");
  let _ = io::stdout().flush();

  io::stdin().read_line(&mut input).expect("Failed to read line");

  let result = basic::add(1.,2.);
  println!("{}", result);
}