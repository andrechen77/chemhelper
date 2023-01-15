use super::{elements::*, formulas::*};
use std::collections::hash_map::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
	Element(&'a Element),
	MolecularFormula(MolecularFormula<'a>),
	Integer(u32),
	String(String),
}

impl<'a> Value<'a> {
	pub fn element(self) -> Option<&'a Element> {
		match self {
			Value::Element(element) => Some(element),
			_ => None,
		}
	}

	pub fn molecular_formula(self) -> Option<MolecularFormula<'a>> {
		match self {
			Value::MolecularFormula(formula) => Some(formula),
			_ => None,
		}
	}

	pub fn integer(self) -> Option<u32> {
		match self {
			Value::Integer(integer) => Some(integer),
			_ => None,
		}
	}

	pub fn string(self) -> Option<String> {
		match self {
			Value::String(string) => Some(string),
			_ => None,
		}
	}
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
			self.values
				.insert(element_info.symbol.clone(), Value::Element(element_info));
		}
	}
}
