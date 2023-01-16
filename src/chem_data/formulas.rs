use crate::chem_data::elements::Element;
use crate::helper::coeff_vec::CoeffVec;
use std::{
	fmt,
	ops::{Add, AddAssign, Mul, MulAssign},
};

#[derive(Debug, PartialEq, Clone)]
pub struct MolecularFormula<'a> {
	element_count: CoeffVec<&'a Element>,
}

impl<'a> MolecularFormula<'a> {
	pub fn new() -> Self {
		MolecularFormula {
			element_count: CoeffVec::new(),
		}
	}

	pub fn set_subscr(&mut self, element: &'a Element, new_subscript: u32) {
		let new_subscript = i32::try_from(new_subscript).unwrap();
		self.element_count.set_coeff(&element, new_subscript);
	}

	pub fn get_subscr(&self, element: &'a Element) -> u32 {
		self.element_count.get_coeff(&element).unsigned_abs()
	}
}

impl<'a> AddAssign for MolecularFormula<'a> {
	fn add_assign(&mut self, rhs: Self) {
		self.element_count += rhs.element_count;
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
		self.element_count *= i32::try_from(rhs).unwrap();
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
		for &(element, subscript) in &self.element_count {
			write!(
				f,
				"{}{}",
				element.symbol,
				if subscript == 1 {
					"".to_string()
				} else {
					subscript.to_string()
				}
			)?;
		}
		Ok(())
	}
}

impl Default for MolecularFormula<'_> {
	fn default() -> Self {
		Self::new()
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use crate::chem_data::{dictionary::Dictionary, elements::PeriodicTable};
// 	use crate::parse::parse_with_dict::*;

// 	#[test]
// 	fn adds_correctly() {
// 		let p_table = PeriodicTable::new_alphabetic();
// 		let mut dict = Dictionary::new();
// 		dict.load_elements(&p_table);

// 		struct Case<'a> {
// 			addend0: MolecularFormula<'a>,
// 			addend1: MolecularFormula<'a>,
// 			sum: MolecularFormula<'a>,
// 		}

// 		let make_case = |addend0: &str, addend1: &str, sum: &str| -> Case {
// 			Case {
// 				addend0: parse_molecular_formula_with_dict(&dict, addend0).unwrap(),
// 				addend1: parse_molecular_formula_with_dict(&dict, addend1).unwrap(),
// 				sum: parse_molecular_formula_with_dict(&dict, sum).unwrap(),
// 			}
// 		};

// 		let cases: Vec<Case> = vec![
// 			make_case(".Al1Bo2", ".Al3Ch4", ".Al4Bo2Ch4"),
// 			make_case(".Al0Bo1", ".Al2Ch3", ".Bo1Al2Ch3"),
// 		];

// 		for Case {
// 			addend0,
// 			addend1,
// 			sum,
// 		} in cases
// 		{
// 			assert_eq!(addend0 + addend1, sum);
// 		}
// 	}

// 	#[test]
// 	fn multiplies_correctly() {
// 		let p_table = PeriodicTable::new_alphabetic();
// 		let mut dict = Dictionary::new();
// 		dict.load_elements(&p_table);

// 		struct Case<'a> {
// 			factor0: MolecularFormula<'a>,
// 			factor1: u32,
// 			product: MolecularFormula<'a>,
// 		}

// 		let make_case = |factor0: &str, factor1: u32, product: &str| -> Case {
// 			Case {
// 				factor0: parse_molecular_formula_with_dict(&dict, factor0).unwrap(),
// 				factor1,
// 				product: parse_molecular_formula_with_dict(&dict, product).unwrap(),
// 			}
// 		};

// 		let cases: Vec<Case> = vec![
// 			make_case(".Al1Bo2", 0, "."),
// 			make_case(".Al1Bo2", 1, ".Al1Bo2"),
// 			make_case(".Al1Bo2", 2, ".Al2Bo4"),
// 			make_case(".Al0Bo2", 0, "."),
// 			make_case(".Al0Bo2", 1, ".Bo2"),
// 			make_case(".Al0Bo2", 2, ".Bo4"),
// 		];

// 		for Case {
// 			factor0,
// 			factor1,
// 			product,
// 		} in cases
// 		{
// 			assert_eq!(factor0 * factor1, product);
// 		}
// 	}
// }
