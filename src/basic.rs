pub fn add(input: Vec<&str>) -> String {
  // convert the input string to floats
  let mut result = String::new();
  let mut floats: Vec<f64> = Vec::new();

  for s in input {
    if let Ok(number) = s.parse::<f64>() {
      floats.push(number);
    } else {
      result = format!("{} is not a valid number.", s);
      break;
    }
  }

  if result.is_empty() {
    let sum: f64 = floats.iter().sum();
    result = format!("{}", sum);
  }

  result
}


pub fn subtract(input: Vec<&str>) -> String {
  // convert the input string to floats
  let mut result = String::new();
  let mut floats: Vec<f64> = Vec::new();

  for s in input {
    if let Ok(number) = s.parse::<f64>() {
      floats.push(number);
    } else {
      result = format!("{} is not a valid number.", s);
      break;
    }
  }

  if result.is_empty() {
    let first: f64 = floats.remove(0);
    let sum: f64 = floats.iter().sum();

    result = format!("{}", first - sum);
  }

  result
}


pub fn check_if_floats(input_vec: Vec<&str>) -> bool {
  for s in input_vec {
    if let Err(_) = s.parse::<f64>() {
      return false;
    }
  }

  true

}