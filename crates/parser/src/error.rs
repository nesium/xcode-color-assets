use std::io;
use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{message}")]
  InvalidColorSetDeclaration { message: String },
  #[error("Error: {message}")]
  ParseError { message: String },
  #[error("{message}")]
  ResolveError { message: String },
  #[error(transparent)]
  Io(#[from] io::Error),
  #[error(transparent)]
  Utf8(#[from] Utf8Error),
}
