use std::io;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Contents of file {path:?} remain identical. The file has not been touched.")]
  FileIsIdentical { path: PathBuf },
  #[error(transparent)]
  Io {
    #[from]
    source: io::Error,
  },
  #[error(transparent)]
  Parser {
    #[from]
    source: parser::Error,
  },
}
