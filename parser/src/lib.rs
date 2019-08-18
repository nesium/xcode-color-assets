pub mod ast;
mod parser;

pub use self::parser::{parse_document, parse_document_from_file};
