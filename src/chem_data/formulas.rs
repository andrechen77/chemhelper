use std::{ fmt};
use crate::chem_data::elements::Element;

#[derive(Debug)]
pub struct MolecularFormula<'a> {
	element_count: Vec<(Element<'a>, i32)>,
}

impl<'a> MolecularFormula<'a> {
	pub fn new() -> Self {
		MolecularFormula {element_count: Vec::new()}
	}

	pub fn set_element_count(&mut self, element: Element<'a>, new_subscript: i32) {
		if let Some((_, subscript)) = self.element_count.iter_mut().find(|(e, _)| *e == element) {
			*subscript = new_subscript;
		} else {
			self.element_count.push((element, new_subscript));
		}
	}

	pub fn get_element_count(&self, element: Element<'a>) -> i32 {
		match self.element_count.iter().find(|(e, _)| *e == element) {
			Some((_, count)) => *count,
			None => 0,
		}
	}
}

// TODO override traits to add and multiply molecular formulas

impl fmt::Display for MolecularFormula<'_> {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Ok(())
	}
}
