use std::io;
use std::io::Write;

mod basic;

fn main() {
  println!("Welcome to math app.");
  println!("Type, \"help\" for a list of commands.");

  // Initialize a variable to store user input
  let mut input = String::new();

  // test our check_if_floats fn
  let floats_string = String::from("1 2 d 3");
  println!("{}", basic::check_if_floats(floats_string.split_whitespace().collect()));


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
        println!("add <num1> <num2> <num3> ...      #adds all the numbers together");
        println!("subtract <num1> <num2> <num3> ... #subtracts all numbers from the first number");
      }
      "hello" => println!("Nice to see you"),
      "add" => println!("{}", basic::add(input_vec)),
      "subtract" => println!("{}", basic::subtract(input_vec)),

      _ => println!("\"{}\" is not a recognized command.", input),
    }

    // reset variable
    input.clear();
  }

}