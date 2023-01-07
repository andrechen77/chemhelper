use std::collections::HashMap;

use super::elements::*;

pub struct MolecularFormula<'a> {
	element_count: HashMap<Element<'a>, i32>,
}

impl MolecularFormula<'_> {
	// TODO add new function

	pub fn get_count(&self, element: &Element) {
		self.element_count.get(element).unwrap_or(&0);
	}
}

// TODO override traits to add and multiply molecular formulas