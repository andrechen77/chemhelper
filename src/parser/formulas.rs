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
	pub fn from_token_iter<I>(p_table: &'a PeriodicTable, token_iter: &mut Peekable<TokenIter<I>>) -> Self
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
					result.set_subscr(element, 1);
					last_symbol = Some(element);
					expectation = SymbolOrSubscript;
				},
				Token::Number(number) => {
					result.set_subscr(last_symbol.expect("Shouldn't have tried to parse a number before a symbol was encountered"), number);
					expectation = Symbol;
				},
				_ => panic!("Shouldn't have tried to process this token if it wasn't expected")
			}
		}
		
		result
	}

	pub fn from_str(p_table: &'a PeriodicTable, formula: &str) -> Self {
		let mut token_iter = TokenIter::from_char_iter(formula.chars()).peekable();
		MolecularFormula::from_token_iter(p_table, &mut token_iter)
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
	use crate::chem_data::std_p_table::std_p_table;

	#[test]
	fn token_iter_method_rejects_double_subscript() {
		let formula = "H12 34";
		let mut input = TokenIter::from_char_iter(formula.chars()).peekable();
		let _formula = MolecularFormula::from_token_iter(std_p_table(), &mut input);
		assert_eq!(*input.peek().unwrap(), Token::Number(34));
	}
}