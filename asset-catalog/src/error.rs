use std::io;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Asset Catalog already exists at path {path:?}")]
  CatalogExists { path: PathBuf },
  #[error(transparent)]
  Io(#[from] io::Error),
  #[error(transparent)]
  Parser(#[from] parser::Error),
}
