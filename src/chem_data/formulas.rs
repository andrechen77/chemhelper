use crate::chem_data::parser::Token;

use super::elements::*;
use super::parser::TokenIter;
use std::{collections::HashMap, iter::Peekable, fmt};

#[derive(Debug)]
pub struct MolecularFormula<'a> {
	element_count: HashMap<Element<'a>, i32>,
}

impl<'a> MolecularFormula<'a> {
	pub fn from<I>(p_table: &'a PeriodicTable, token_iter: &mut Peekable<TokenIter<I>>) -> Self
	where
		I: Iterator<Item = char>
	{
		enum Expectation {
			SymbolOrSubscript,
			Symbol,
		}
		use Expectation::*;
		let is_compatible = |expectation: &Expectation, token: &Token| -> bool {
			match token {
				Token::Symbol(symbol) => p_table.get_element(symbol).is_some(),
				Token::Number(_) => {
					if let SymbolOrSubscript = expectation { true } else { false }
				},
				_ => false,
			}
		};
		
		let mut result = MolecularFormula {element_count: HashMap::new()};

		let mut expectation = Symbol;
		let mut last_symbol: Option<Element> = None;
		while let Some(token) = token_iter.next_if(|t| is_compatible(&expectation, &t)) {
			match token {
				Token::Symbol(symbol) => {
					let element = p_table.get_element(&symbol)
						.expect("Shouldn't have tried to parse a symbol if it wasn't compatible");
					result.add_element(element, 1);
					last_symbol = Some(element);
					expectation = SymbolOrSubscript;
				},
				Token::Number(number) => {
					let subscript = result.element_count
						.get_mut(&last_symbol.expect("Shouldn't have tried to parse a number before a symbol was encountered."))
						.expect("The last symbol encountered should already be in the element counts");
					*subscript = number;
					expectation = Symbol;
				},
				_ => panic!("Shouldn't have tried to process this token if it wasn't expected")
			}
		}
		
		result
	}

	fn add_element(&mut self, element: Element<'a>, count: i32) {
		*self.element_count.entry(element).or_insert(0) += count;
	}

	pub fn get_count(&self, element: &Element) {
		self.element_count.get(element).unwrap_or(&0);
	}
}

// TODO override traits to add and multiply molecular formulas

impl fmt::Display for MolecularFormula<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rejects_double_subscript() {
		let p_table = PeriodicTable::from(std::fs::read_to_string("ptable.txt").unwrap());
		let mut input = TokenIter::from_char_iter("H12 34".chars()).peekable();
		let _formula = MolecularFormula::from(&p_table, &mut input);
		assert_eq!(*input.peek().unwrap(), Token::Number(34));
	}
}