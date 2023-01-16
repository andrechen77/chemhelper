// use super::expression::parser::{self, ParseError};
// use crate::chem_data::{dictionary::Dictionary, elements::Element, formulas::MolecularFormula};

// pub fn parse_element_with_dict<'a>(
// 	dict: &'a Dictionary,
// 	string: &str,
// ) -> Result<&'a Element, ParseError> {
// 	Ok(parser::parse_str(string)?.evaluate(dict)?.element()?)
// }

// pub fn parse_molecular_formula_with_dict<'a>(
// 	dict: &'a Dictionary,
// 	string: &str,
// ) -> Result<MolecularFormula<'a>, ParseError> {
// 	Ok(parser::parse_str(string)?.evaluate(dict)?.element()?)
// }
