use super::tokens::{IntoTokenIter, Token, Tokens};
use crate::chem_data::dictionary::{self, Dictionary};
use crate::chem_data::elements::*;
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

	fn skip_leading_whitespace(&mut self) {
		while self.tokens.peek(0) == Some(&Token::Whitespace) {
			self.tokens.next();
		}
	}

	/// Takes an element from the given stream of tokens. An element can be represented as an
	/// identifier mapping to Value::Element in the dictionary. If an unexpected token was found,
	/// consumes that token and reports it as an error.
	pub fn expect_element<'b>(
		&mut self,
		dict: &'b Dictionary,
	) -> Result<Element<'b>, ParseError<'b>> {
		self.skip_leading_whitespace();
		let next_token = self.tokens.next().ok_or(ParseError::EndOfStream)?;
		match next_token {
			Token::Identifier(name) => match dict.get_value(&name) {
				Some(dictionary::Value::Element(element)) => Ok(element.to_owned()),
				Some(value) => Err(ParseError::IdentifierBadType(name, value)),
				None => Err(ParseError::EndOfStream),
			},
			token => Err(ParseError::UnexpectedSyntax(token)),
		}
	}

	// pub fn get_molecular_formula<'b>(&mut self, dict: &Dictionary) -> Option<MolecularFormula<'b>> {
	// 	// check the first character is a '.'
	// 	let first_char = self.tokens.next()?;
	// 	if first_char != Token::Dot {
	// 		self.tokens.put_back(first_char);
	// 		return None;
	// 	}

	// 	enum State {
	// 		AfterSymbol,
	// 		AfterSymbolOrSubscript,
	// 	}

	// 	let mut result = MolecularFormula::new();
	// 	let mut state = State::AfterSymbolOrSubscript;
	// 	while let Some(token) = self.tokens.next() {
	// 		match state {
	// 			State::AfterSymbolOrSubscript => {
	// 				match token {
	// 					Token::Identifier(name) =>
	// 				}
	// 			}
	// 			State::AfterSymbol => {

	// 			},
	// 		}
	// 	}
	// 	todo!();

	// 	// TODO add optional charges
	// }
}

impl<'a> From<&'a str> for Parser<std::str::Chars<'a>> {
	fn from(value: &'a str) -> Self {
		Self::new(value.chars().into_token_iter())
	}
}

#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
	IdentifierBadType(String, &'a dictionary::Value<'a>),
	UndefinedIdentifier(String),
	UnexpectedSyntax(Token),
	EndOfStream,
}

impl Display for ParseError<'_> {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_elements_correctly() {
		let p_table = PeriodicTable::new_alphabetic();
		let mut dict = Dictionary::new();
		dict.load_elements(&p_table);

		let mut parser = Parser::from("AlBo Ch;Dv");
		assert_eq!(
			parser.expect_element(&dict),
			Ok(p_table.get_element("Al").unwrap())
		);
		assert_eq!(
			parser.expect_element(&dict),
			Ok(p_table.get_element("Bo").unwrap())
		);
		assert_eq!(
			parser.expect_element(&dict),
			Ok(p_table.get_element("Ch").unwrap())
		);
		assert_eq!(
			parser.expect_element(&dict),
			Err(ParseError::UnexpectedSyntax(Token::Unknown(
				";".to_string()
			)))
		);
		assert_eq!(
			parser.expect_element(&dict),
			Ok(p_table.get_element("Dv").unwrap())
		);
	}
}
