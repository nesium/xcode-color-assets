pub mod ast;
mod parser;
mod var_context;
mod error;

pub use self::parser::{parse_document, parse_document_from_file};
pub use self::var_context::{VarContext, ResolvedVariable, ResolvedColorSet};
pub use self::error::Error;
