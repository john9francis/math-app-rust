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


pub fn string_to_floats(floats_string : String) -> Result<Vec<f64>, Vec<String>> {
  // convert input string to a vec
  let mut input_vec: Vec<&str> = floats_string.split_whitespace().collect();

  // convert the input string to floats
  let mut floats: Vec<f64> = Vec::new();
  let mut invalid_floats: Vec<String> = Vec::new();

  for s in input_vec {
    if let Ok(number) = s.parse::<f64>() {

      // if it can be converted, add to floats list
      floats.push(number);

    } else {

      // if it can't be converted to a string, add to the invalid floats
      invalid_floats.push(s.to_string());      

    }
  }

    if invalid_floats.is_empty() {
      Err(invalid_floats)
    } else {
      Ok(floats)
    }
  
}