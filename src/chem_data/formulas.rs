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

impl<'a> MulAssign<u32> for MolecularFormula<'a> {
	fn mul_assign(&mut self, rhs: u32) {
		if rhs == 0 {
			self.element_count.clear();
		}

		for (_, subscript) in &mut self.element_count {
			*subscript *= rhs;
		}
	}
}

impl<'a> Mul<u32> for MolecularFormula<'a> {
	type Output = Self;

	fn mul(mut self, rhs: u32) -> Self::Output {
		self *= rhs;
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
	use crate::chem_data::elements::PeriodicTable;

	#[test]
	fn adds_correctly() {
		let p_table = PeriodicTable::new_alphabetic();

		struct Case<'a> {
			addend0: MolecularFormula<'a>,
			addend1: MolecularFormula<'a>,
			sum: MolecularFormula<'a>,
		}

		let make_case = |addend0: &str, addend1: &str, sum: &str| -> Case {
			Case {
				addend0: MolecularFormula::from_str(&p_table, addend0),
				addend1: MolecularFormula::from_str(&p_table, addend1),
				sum: MolecularFormula::from_str(&p_table, sum),
			}
		};

		let cases: Vec<Case> = vec![
			make_case("A1B2", "A3C4", "A4B2C4"),
			make_case("A0B1", "A2C3", "B1A2C3"),
		];

		for Case {addend0, addend1, sum} in cases {
			assert_eq!(addend0 + addend1, sum);
		}
	}

	#[test]
	fn multiplies_correctly() {
		let p_table = PeriodicTable::new_alphabetic();

		struct Case<'a> {
			factor0: MolecularFormula<'a>,
			factor1: u32,
			product: MolecularFormula<'a>,
		}

		let make_case = |factor0: &str, factor1: u32, product: &str| -> Case {
			Case {
				factor0: MolecularFormula::from_str(&p_table, factor0),
				factor1,
				product: MolecularFormula::from_str(&p_table, product),
			}
		};

		let cases: Vec<Case> = vec![
			make_case("A1B2", 0, ""),
			make_case("A1B2", 1, "A1B2"),
			make_case("A1B2", 2, "A2B4"),
			make_case("A0B2", 0, ""),
			make_case("A0B2", 1, "B2"),
			make_case("A0B2", 2, "B4"),
		];

		for Case {factor0, factor1, product} in cases {
			assert_eq!(factor0 * factor1, product);
		}
	}
}