use super::tokens::{Token, Tokens};
use crate::chem_data::dictionary::*;
use crate::chem_data::elements::*;
use crate::helper::peek_iter::PeekIter;

/// An adapter on a Token iterator that parses the tokens into chem_data objects
pub struct Parser<'a, I: Iterator<Item = char>> {
    tokens: PeekIter<Tokens<'a, I>>,
}

impl<'a, I: Iterator<Item = char>> Parser<'a, I> {
    pub fn new(tokens: Tokens<'a, I>) -> Self {
        Parser {
            tokens: PeekIter::new(tokens),
        }
    }

    pub fn get_raw_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Takes an element from the given stream of tokens. An element can be represented as an
    /// identifier mapping to Value::Element in the dictionary. Returns None and consumes no tokens
    /// if no element was found.
    pub fn expect_element<'b>(&mut self, dict: &'b Dictionary) -> Option<Element<'b>> {
        let peek_token = self.tokens.peek(0)?;
        if let Token::Identifier(name) = peek_token {
            if let Value::Element(element) = dict.get_value(name)? {
                self.tokens.next();
                return Some(element.to_owned());
            }
        }
        None
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
