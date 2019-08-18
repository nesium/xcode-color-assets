use std::fmt;

#[derive(Debug)]
pub enum Error {
  CatalogExists(String),
  CouldNotCreateFile(String),
  CouldNotCreateDirectory(String),
  CouldNotRemoveDirectory(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let message = match self {
      Error::CatalogExists(path) => format!("Asset Catalog already exists at path {}.", path),
      Error::CouldNotCreateFile(path) => format!("Could not create file at path {}.", path),
      Error::CouldNotCreateDirectory(path) => {
        format!("Could not create directory at path {}.", path)
      }
      Error::CouldNotRemoveDirectory(path) => {
        format!("Could not remove directory at path {}.", path)
      }
    };
    write!(f, "Error: {}", message)
  }
}
