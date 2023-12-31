pub fn add(input: Vec<&str>) -> String {

  // process the numbers to see if they are floats
  if let Ok(floats) = process_numbers(&input) {

      // add up and return a string with the sum  
      let sum: f64 = floats.iter().sum();
      return format!("{}", sum);

  } else {

      // get the character that isn't a float
      return process_numbers(&input).unwrap_err();
  }
}



pub fn subtract(input: Vec<&str>) -> String {

  // check if the input can be turned to floats
  if let Ok(mut floats) = process_numbers(&input) {

      // subtract all the floats from the first one
      let first: f64 = floats.remove(0);
      let sum: f64 = floats.iter().sum();

      // return a string of the result
      return format!("{}", first - sum);

  } else {

      // get the character that is causing the error
      return process_numbers(&input).unwrap_err();
  }
}



fn process_numbers(input: &Vec<&str>) -> Result<Vec<f64>, String> {

  // take in strings and make sure they're all floats

  let mut floats: Vec<f64> = Vec::new();

  for s in input {
      if let Ok(number) = s.parse::<f64>() {
          floats.push(number);
      } else {
          return Err(format!("{} is not a valid number.", s));
      }
  }

  Ok(floats)
}