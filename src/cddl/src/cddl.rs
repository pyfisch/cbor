use pest::iterators::Pairs;
use pest::Parser;

pub mod error;
mod parser;
pub mod ruleset;

pub use ruleset::Ruleset;

pub fn parse(content: &str) -> Result<Pairs<parser::Rule>, pest::error::Error<parser::Rule>> {
    parser::CDDLParser::parse(parser::Rule::cddl, content)
}
