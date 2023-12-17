pub fn add(input: &str) -> &str {
  // convert the input string to integers
  let mut result = "";
  
  let strings : Vec<&str> = input.split_whitespace().collect();
  let mut floats : Vec<f64> = Vec::new();

  for s in &strings {
    if let Ok(number) = s.parse::<f64>() {
      floats.push(number)
    } else {
      result = &format!("{} is not a valid number.", s);
    }
  }

  if result == "" {
    &format!("{:?}", floats);
  }
  

  result
}
