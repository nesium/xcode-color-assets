use std::fmt;

#[derive(Debug)]
pub enum Error {
  FileIsIdentical(String),
  IO(String),
  VariableLookupFailure(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let message = match self {
      Self::FileIsIdentical(path) => format!(
        "Contents of file {} remain identical. The file has not been touched.",
        path
      ),
      Self::IO(msg) => msg.to_string(),
      Self::VariableLookupFailure(msg) => msg.to_string(),
    };
    write!(f, "{}", message)
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    use std::error::Error;
    Self::IO(error.description().to_string())
  }
}

impl From<parser::Error> for Error {
  fn from(error: parser::Error) -> Self {
    use std::error::Error;
    Self::VariableLookupFailure(error.description().to_string())
  }
}
