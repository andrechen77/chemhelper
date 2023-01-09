use std::{
	fmt,
	ops::{Add, AddAssign, Mul, MulAssign},
};
use crate::chem_data::{
	elements::Element
};

#[derive(Debug, PartialEq)]
pub struct MolecularFormula<'a> {
	element_count: Vec<(Element<'a>, u32)>,
}

impl<'a> MolecularFormula<'a> {
	pub fn new() -> Self {
		MolecularFormula {element_count: Vec::new()}
	}

	pub fn set_subscr(&mut self, element: Element<'a>, new_subscript: u32) {
		if new_subscript == 0 {
			self.element_count.retain(|(e, _)| *e != element);
			return;
		}

		if let Some((_, subscript)) = self.element_count.iter_mut().find(|(e, _)| *e == element) {
			*subscript = new_subscript;
		} else {
			self.element_count.push((element, new_subscript));
		}
	}

	pub fn get_subscr(&self, element: Element<'a>) -> u32 {
		match self.element_count.iter().find(|(e, _)| *e == element) {
			Some((_, count)) => *count,
			None => 0,
		}
	}
}

// TODO override traits to add and multiply molecular formulas

impl<'a> AddAssign for MolecularFormula<'a> {
	fn add_assign(&mut self, rhs: Self) {
		for (element, subscript) in rhs.element_count {
			self.set_subscr(
				element,
				self.get_subscr(element) + subscript
			);
		}
	}
}

impl<'a> Add for MolecularFormula<'a> {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl fmt::Display for MolecularFormula<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (element, subscript) in &self.element_count {
			write!(
				f,
				"{}{}",
				element.identity.symbol,
				if *subscript == 1 {"".to_string()} else {subscript.to_string()}
			)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::chem_data::std_p_table::std_p_table;

	#[test]
	fn adds_correctly() {
		struct Case<'a> {
			addend0: MolecularFormula<'a>,
			addend1: MolecularFormula<'a>,
			sum: MolecularFormula<'a>,
		}

		fn make_case<'a>(addend0: &'a str, addend1: &'a str, sum: &'a str) -> Case<'a> {
			Case {
				addend0: MolecularFormula::from_str(std_p_table(), addend0),
				addend1: MolecularFormula::from_str(std_p_table(), addend1),
				sum: MolecularFormula::from_str(std_p_table(), sum),
			}
		}

		let cases: Vec<Case> = vec![
			make_case("H1O2", "H3N4", "H4O2N4"),
			make_case("H0O1", "H2N3", "O1H2N3"),
		];

		for Case {addend0, addend1, sum} in cases {
			assert_eq!(addend0 + addend1, sum);
		}
	}
}