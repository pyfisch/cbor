use crate::cddl::parser;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

impl From<pest::error::Error<parser::Rule>> for Error {
    fn from(err: pest::error::Error<parser::Rule>) -> Self {
        Error::ParseError(format!("{}", err))
    }
}
