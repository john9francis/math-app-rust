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

    // make the input into a vector
    let mut input_vec: Vec<&str> = input.split_whitespace().collect();
    let command = input_vec.remove(0);

    // handle inputs
    match command {
      "quit" => {
        println!("quitting...");
        break;
      },
      "help" => {
        println!("List of available commands: ");
        println!("quit                              #exits the program");
        println!("hello                             #greets you");
        println!("add <number1> <number2>           #adds number1 to number2");
      }
      "hello" => println!("Nice to see you"),
      "add" => println!("{}", basic::add("1 2 3")),

      _ => println!("\"{}\" is not a recognized command.", input),
    }

    // reset variable
    input.clear();
  }

}