use super::{elements::*, formulas::*};
use std::collections::hash_map::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
	Element(Element<'a>),
	MolecularFormula(MolecularFormula<'a>),
	Integer(u32),
}

#[derive(Debug)]
pub struct Dictionary<'a> {
	values: HashMap<String, Value<'a>>,
}

impl<'a> Dictionary<'a> {
	pub fn new() -> Self {
		Dictionary {
			values: HashMap::new(),
		}
	}

	pub fn get_value(&self, name: &str) -> Option<&Value<'a>> {
		self.values.get(name)
	}

	pub fn load_elements(&mut self, p_table: &'a PeriodicTable) {
		for element_info in p_table {
			self.values.insert(
				element_info.symbol.clone(),
				Value::Element(Element::new(element_info)),
			);
		}
	}
}
