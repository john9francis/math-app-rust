use std::io;
use std::io::Write;

mod basic;

fn main() {
  println!("Hello Math app world!");

  // get input from user
  let mut input = String::new();

  loop {
    print!("Input: ");
    let _ = io::stdout().flush();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    // clean up the input
    if let Some('\n') = input.chars().next_back() {
      input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
      input.pop();
    }

    // let user quit out of loop
    if input == "q" {
      println!("quitting...");
      break;
    }

    println!("{}", input);

    // reset variable
    input.clear();
  }

  let result = basic::add(1.,2.);
  println!("{}", result);
}