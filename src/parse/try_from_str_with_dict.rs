use super::parser::{ParseError, Parser};
use crate::chem_data::{dictionary::Dictionary, elements::Element, formulas::MolecularFormula};

pub trait FromStrWithDict<'a>: Sized {
	fn from_str_w_dict(dict: &'a Dictionary, string: &str) -> Result<Self, ParseError>;
}

impl<'a> FromStrWithDict<'a> for Element<'a> {
	fn from_str_w_dict(dict: &'a Dictionary, string: &str) -> Result<Self, ParseError> {
		Parser::from(string).parse_element(dict)
	}
}

impl<'a> FromStrWithDict<'a> for MolecularFormula<'a> {
	fn from_str_w_dict(dict: &'a Dictionary, string: &str) -> Result<Self, ParseError> {
		Parser::from(string).parse_molecular_formula(dict)
	}
}
