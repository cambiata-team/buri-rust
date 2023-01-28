use crate::document::document;
use ast::{DocumentNode, ParserInput};
use nom::{combinator::eof, sequence::terminated};

pub fn parse_buri_file(source: &str) -> Result<DocumentNode, String> {
    let input = ParserInput::new(source);
    let result = terminated(document(), eof)(input);
    match result {
        Ok((_, document)) => Ok(document),
        Err(error) => Err(error.to_string()),
    }
}
