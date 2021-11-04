use std::{fmt, fs, io};

pub enum InputType {
  Example,
  Challenge,
}
impl fmt::Display for InputType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let formatted = match *self {
      InputType::Challenge => "challenge",
      InputType::Example => "example",
    };
    write!(f, "{}", formatted)
  }
}

pub fn get_input(year: u16, day: u8, input_type: InputType, input_num: u8) -> io::Result<String> {
  let filepath = format!("input/{}/{}/{}_{}", year, day, input_type, input_num);

  fs::read_to_string(&filepath)
}

#[cfg(test)]
mod test {
  use super::{get_input, InputType};
  #[test]
  fn missing_file() {
    let input = get_input(0, 0, InputType::Example, 0);
    assert_eq!(input.is_err(), true);
  }

  #[test]
  fn example_input() {
    let input = get_input(2021, 0, InputType::Example, 0);
    assert_eq!(input.is_ok(), true);
    assert_eq!(input.unwrap(), "test\n");
  }
}
