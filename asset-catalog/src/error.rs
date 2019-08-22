use std::fmt;

#[derive(Debug)]
pub enum Error {
  CatalogExists(String),
  CouldNotCreateFile(String),
  CouldNotCreateDirectory(String),
  CouldNotRemoveDirectory(String),
  UnknownIdentifier(String),
  AssignColorSetToLightProperty(String),
  AssignColorSetToDarkProperty(String),
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
      Self::UnknownIdentifier(identifier) => {
        format!("Could not find variable with identifier {}.", identifier)
      }
      Self::AssignColorSetToLightProperty(identifier) => format!(
        "Attempt to assign a colorset to the light property of another colorset via variable {}.",
        identifier
      ),
      Self::AssignColorSetToDarkProperty(identifier) => format!(
        "Attempt to assign a colorset to the dark property of another colorset via variable {}.",
        identifier
      ),
    };
    write!(f, "Error: {}", message)
  }
}
