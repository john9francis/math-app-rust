pub fn add(input: Vec<&str>) -> String {

  // process the numbers to see if they are floats
  if let Ok(floats) = convert_strings_to_floats(&input) {

      // add up and return a string with the sum  
      let sum: f64 = floats.iter().sum();
      return format!("{}", sum);

  } else {

      // get the character that isn't a float
      return convert_strings_to_floats(&input).unwrap_err();
  }
}



pub fn subtract(input: Vec<&str>) -> String {

  // check if the input can be turned to floats
  if let Ok(mut floats) = convert_strings_to_floats(&input) {

      // subtract all the floats from the first one
      let first: f64 = floats.remove(0);
      let sum: f64 = floats.iter().sum();

      // return a string of the result
      return format!("{}", first - sum);

  } else {

      // get the character that is causing the error
      return convert_strings_to_floats(&input).unwrap_err();
  }
}



pub fn multiply(input: Vec<&str>) -> String {
  // check if the input can be turned to floats
  if let Ok(floats) = convert_strings_to_floats(&input) {

    // multiply all floats together
    let product: f64 = floats.iter().product();

    // return a string of the result
    return format!("{}", product);

  } else {

    // get the character that is causing the error
    return convert_strings_to_floats(&input).unwrap_err();
  }
}

pub fn divide(input: Vec<&str>) -> String {

  // check if the input can be turned to floats
  if let Ok(mut floats) = convert_strings_to_floats(&input) {

      // divide the first value by the product of all the rest.
      let first: f64 = floats.remove(0);
      let product: f64 = floats.iter().product();

      // return a string of the result
      return format!("{}", first / product);

  } else {

      // get the character that is causing the error
      return convert_strings_to_floats(&input).unwrap_err();
  }
}

pub fn factorial(input: Vec<&str>) -> String {
  // first, make sure vec is size 1
  if input.len() != 1 {
    return "factorial function takes in exactly one number.".to_string();
  }

  // make sure input is a number
  let floats = match convert_strings_to_floats(&input) {
    Ok(floats) => floats,
    Err(e) => return e,
  };

  // make sure input is non negative
  let num = floats[0];
  if num < 0.0 {
    return format!("input: {}, can not compute factorial of a negative number", num)
  }

  // if everything is fine up to this point, calc the factorial
  format!("{}", recursive_factorial(num))

}

fn recursive_factorial(n: f64) -> f64 {
  if n <= 0.0 {
    1.0
  } else {
    n * recursive_factorial(n - 1.0)
  }
}



fn convert_strings_to_floats(input: &Vec<&str>) -> Result<Vec<f64>, String> {

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