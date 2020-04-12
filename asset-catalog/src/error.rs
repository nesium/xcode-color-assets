use std::fmt;

#[derive(Debug)]
pub enum Error {
  CatalogExists(String),
  CouldNotCreateFile(String),
  CouldNotCreateDirectory(String),
  CouldNotRemoveDirectory(String),
  VariableLookupFailure(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let message = match self {
      Self::CatalogExists(path) => format!("Asset Catalog already exists at path {}.", path),
      Self::CouldNotCreateFile(path) => format!("Could not create file at path {}.", path),
      Self::CouldNotCreateDirectory(path) => {
        format!("Could not create directory at path {}.", path)
      }
      Self::CouldNotRemoveDirectory(path) => {
        format!("Could not remove directory at path {}.", path)
      }
      Self::VariableLookupFailure(message) => message.to_string(),
    };
    write!(f, "Error: {}", message)
  }
}

impl From<parser::Error> for Error {
  fn from(error: parser::Error) -> Self {
    Self::VariableLookupFailure(error.to_string())
  }
}
