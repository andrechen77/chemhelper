use std::iter::Peekable;
use crate::chem_data::{
	formulas::MolecularFormula,
	elements::*,
};
use super::{
	Token,
	TokenIter,
};

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
				Token::Number(_) =>	matches!(expectation, SymbolOrSubscript),
				_ => false,
			}
		};
		
		let mut result = MolecularFormula::new();

		let mut expectation = Symbol;
		let mut last_symbol: Option<Element> = None;
		while let Some(token) = token_iter.next_if(|t| is_compatible(&expectation, t)) {
			match token {
				Token::Symbol(symbol) => {
					let element = p_table.get_element(&symbol)
						.expect("Shouldn't have tried to parse a symbol if it wasn't compatible");
					result.set_element_count(element, 1);
					last_symbol = Some(element);
					expectation = SymbolOrSubscript;
				},
				Token::Number(number) => {
					result.set_element_count(last_symbol.expect("Shouldn't have tried to parse a number before a symbol was encountered"), number);
					expectation = Symbol;
				},
				_ => panic!("Shouldn't have tried to process this token if it wasn't expected")
			}
		}
		
		result
	}
}

impl<'a> Default for MolecularFormula<'a> {
	fn default() -> Self {
		Self::new()
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