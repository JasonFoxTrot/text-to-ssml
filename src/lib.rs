#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
extern crate quick_xml;

pub mod errors;
pub mod parser;
pub mod ssml_constants;
pub mod xml_writer;

use ::errors::*;

/// Parses a String into the Unique Text to SSML Format. Useful for taking a string
/// and making some sweet, sweet SSML.
pub fn parse_string(to_parse: String) -> Result<String> {
  parser::parse_as_ssml(to_parse)
}
