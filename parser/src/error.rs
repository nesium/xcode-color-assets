use std::fmt;

#[derive(Debug)]
pub struct Error {
  error: String,
}

impl Error {
  pub fn new(error: String) -> Self {
    Error { error }
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    &self.error
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error: {}", self.error)
  }
}
