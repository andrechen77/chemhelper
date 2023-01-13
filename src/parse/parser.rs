use super::tokens::{IntoTokenIter, Token, Tokens};
use crate::chem_data::dictionary::{self, Dictionary};
use crate::chem_data::{elements::Element, formulas::MolecularFormula};
use crate::helper::peek_iter::PeekIter;
use std::fmt::Display;

/// An adapter on a Token iterator that parses the tokens into chem_data objects
pub struct Parser<I: Iterator<Item = char>> {
	tokens: PeekIter<Tokens<I>>,
}

impl<I: Iterator<Item = char>> Parser<I> {
	pub fn new(tokens: Tokens<I>) -> Self {
		Parser {
			tokens: PeekIter::new(tokens),
		}
	}

	pub fn get_raw_token(&mut self) -> Option<Token> {
		self.tokens.next()
	}

	pub fn skip_leading_whitespace(&mut self) {
		while self.tokens.peek(0) == Some(&Token::Whitespace) {
			self.tokens.next();
		}
	}

	/// Takes an element from the given stream of tokens. An element can be represented as an
	/// identifier mapping to Value::Element in the dictionary. If an unexpected token was found,
	/// does not consume that token and reports an error.
	pub fn parse_element<'a>(&mut self, dict: &'a Dictionary) -> Result<&'a Element, ParseError> {
		let next_token = self.tokens.peek(0).ok_or(ParseError::EndOfStream)?;
		match next_token {
			Token::Identifier(name) => match dict.get_value(name) {
				Some(dictionary::Value::Element(element)) => {
					self.tokens.next();
					Ok(*element)
				},
				Some(_) | None => Err(ParseError::InvalidIdentifier(next_token.clone())),
			},
			token => Err(ParseError::UnexpectedSyntax(token.clone())),
		}
	}

	pub fn parse_integer(&mut self, dict: &Dictionary) -> Result<u32, ParseError> {
		let next_token = self.tokens.peek(0).ok_or(ParseError::EndOfStream)?;
		match next_token {
			Token::Identifier(name) => match dict.get_value(name) {
				Some(dictionary::Value::Integer(_)) => {
					Ok(if let Some(Token::Integer(num)) = self.tokens.next() {
						num
					} else {
						unreachable!()
					})
				},
				Some(_) | None => Err(ParseError::InvalidIdentifier(next_token.clone())),
			},
			Token::Integer(_) => Ok(if let Some(Token::Integer(num)) = self.tokens.next() {
				num
			} else {
				unreachable!()
			}),
			token => Err(ParseError::UnexpectedSyntax(token.clone())),
		}
	}

	/// Parses a molecular formula from the stream of tokens. A molecular formula can be represented
	/// as a '.' followed by 0 or more identifers with optional subscripts, or an identifier mapping
	/// to a molecular formula in the dictionary. Stops consuming tokens once a token is encountered
	/// that does not fit the molecular formula syntax (this token is not consumed). If no
	/// molecular formula could be parsed, reports an error.
	pub fn parse_molecular_formula<'a>(
		&mut self,
		dict: &'a Dictionary,
	) -> Result<MolecularFormula<'a>, ParseError> {
		let next_token = self.tokens.peek(0).ok_or(ParseError::EndOfStream)?;
		match next_token {
			Token::Identifier(ref name) => match dict.get_value(name) {
				Some(dictionary::Value::MolecularFormula(formula)) => {
					self.tokens.next();
					Ok(formula.to_owned())
				},
				Some(_) | None => Err(ParseError::InvalidIdentifier(next_token.clone())),
			},
			Token::Dot => {
				self.tokens.next();
				let mut result = MolecularFormula::new();
				loop {
					match self.parse_element(dict) {
						Err(ParseError::UnexpectedSyntax(_) | ParseError::EndOfStream) => break,
						Err(error @ ParseError::InvalidIdentifier(_)) => return Err(error),
						Ok(element) => {
							let subscript = if let Ok(subscript) = self.parse_integer(dict) {
								subscript
							} else {
								1
							};
							result.set_subscr(element, subscript);
						},
					}
				}
				Ok(result)
			},
			token => Err(ParseError::UnexpectedSyntax(token.clone())),
		}
		// TODO add optional charges
	}
}

impl<'a> From<&'a str> for Parser<std::str::Chars<'a>> {
	fn from(value: &'a str) -> Self {
		Self::new(value.chars().into_token_iter())
	}
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
	InvalidIdentifier(Token),
	UnexpectedSyntax(Token),
	EndOfStream,
}

impl Display for ParseError {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::chem_data::elements::PeriodicTable;

	#[test]
	fn parses_elements_correctly() {
		let p_table = PeriodicTable::new_alphabetic();
		let mut dict = Dictionary::new();
		dict.load_elements(&p_table);

		let element = |sym| p_table.get_element(sym).unwrap();

		let mut parser = Parser::from("AlBo Ch;Dv");
		assert_eq!(parser.parse_element(&dict), Ok(element("Al")));
		assert_eq!(parser.parse_element(&dict), Ok(element("Bo")));
		parser.skip_leading_whitespace();
		assert_eq!(parser.parse_element(&dict), Ok(element("Ch")));
		assert_eq!(
			parser.parse_element(&dict),
			Err(ParseError::UnexpectedSyntax(Token::Unknown(
				";".to_string()
			)))
		);
		parser.get_raw_token();
		assert_eq!(parser.parse_element(&dict), Ok(element("Dv")));
	}

	#[test]
	fn parses_molecular_formulas_correctly() {
		let p_table = PeriodicTable::new_alphabetic();
		let mut dict = Dictionary::new();
		dict.load_elements(&p_table);

		let element = |sym| p_table.get_element(sym).unwrap();

		let mut formula_a = MolecularFormula::new();
		formula_a.set_subscr(&element("Al"), 1);
		formula_a.set_subscr(&element("Ch"), 3);
		formula_a.set_subscr(&element("Bo"), 1);

		let mut formula_b = MolecularFormula::new();
		formula_b.set_subscr(&element("Dv"), 2);
		formula_b.set_subscr(&element("Er"), 4);

		let formula_c = MolecularFormula::new();

		let mut parser = Parser::from(".AlCh3Bo .Dv2Er4 . hey.Al2C");
		assert_eq!(parser.parse_molecular_formula(&dict), Ok(formula_a));
		parser.skip_leading_whitespace();
		assert_eq!(parser.parse_molecular_formula(&dict), Ok(formula_b));
		parser.skip_leading_whitespace();
		assert_eq!(parser.parse_molecular_formula(&dict), Ok(formula_c));
		parser.skip_leading_whitespace();
		assert_eq!(
			parser.parse_molecular_formula(&dict),
			Err(ParseError::InvalidIdentifier(Token::Identifier(
				"hey".to_string()
			)))
		);
		parser.get_raw_token();
		assert_eq!(
			parser.parse_molecular_formula(&dict),
			Err(ParseError::InvalidIdentifier(Token::Identifier(
				"C".to_string()
			)))
		);
		assert_eq!(
			parser.get_raw_token().unwrap(),
			Token::Identifier("C".to_string())
		);
	}
}
