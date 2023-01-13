use super::parser::{ParseError, Parser};
use crate::chem_data::{dictionary::Dictionary, elements::Element, formulas::MolecularFormula};

pub fn parse_element_with_dict<'a>(
	dict: &'a Dictionary,
	string: &str,
) -> Result<&'a Element, ParseError> {
	Parser::from(string).parse_element(dict)
}

pub fn parse_molecular_formula_with_dict<'a>(
	dict: &'a Dictionary,
	string: &str,
) -> Result<MolecularFormula<'a>, ParseError> {
	Parser::from(string).parse_molecular_formula(dict)
}
