use std::io;
use std::io::Write;

mod basic;

fn main() {
  println!("Welcome to math app.");
  println!("Type, \"help\" for a list of commands.");

  // Initialize a variable to store user input
  let mut input = String::new();

  loop {
    // get input from user
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
    else if input == "help" {
      println!("Try 'add 1 2' to add 1 to 2");
    }
    else {
      println!("\"{}\" is not a recognized input.", input);
    }


    // reset variable
    input.clear();
  }

  let result = basic::add(1.,2.);
  println!("{}", result);
}