use super::{
	real_number::RealNumber,
	elements::{PeriodicTable, Element},
	formulas::MolecularFormula,
	chem_eqn::ChemEqn,
};
use std::{collections::hash_map::HashMap, error::Error, fmt::{Display, Debug}};

#[derive(Debug)]
pub enum DataType {
	String,
	Integer,
	RealNumber,
	ElementRef,
	MolecularFormula,
	ChemEqn,
}
impl Display for DataType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DataType::String => write!(f, "string"),
			DataType::Integer => write!(f, "integer"),
			DataType::RealNumber => write!(f, "real number"),
			DataType::ElementRef => write!(f, "element reference"),
			DataType::MolecularFormula => write!(f, "molecular formula"),
			DataType::ChemEqn => write!(f, "chemical equation"),
		}
	}
}
impl From<&Value<'_>> for DataType {
	fn from(value: &Value<'_>) -> Self {
		match value {
			Value::String(_) => DataType::String,
			Value::Integer(_) => DataType::Integer,
			Value::RealNumber(_) => DataType::RealNumber,
			Value::ElementRef(_) => DataType::ElementRef,
			Value::MolecularFormula(_) => DataType::MolecularFormula,
			Value::ChemEqn(_) => DataType::ChemEqn,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
	String(String),
	Integer(u32),
	RealNumber(RealNumber),
	ElementRef(&'a Element),
	MolecularFormula(MolecularFormula<'a>),
	ChemEqn(ChemEqn<'a>),
}
impl<'a> Value<'a> {
	pub fn as_type<T: ExpectFromValue<'a>>(self) -> Result<T, BadTypeError<'a>> {
		ExpectFromValue::expect_from_value(self)
	}
}
impl Display for Value<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}) ", DataType::from(self))?;
		match self {
			Value::String(string) => Display::fmt(string, f),
			Value::Integer(integer) => Display::fmt(integer, f),
			Value::RealNumber(real) => Display::fmt(real, f),
			Value::ElementRef(element) => Display::fmt(element, f),
			Value::MolecularFormula(formula) => Display::fmt(formula, f),
			Value::ChemEqn(eqn) => Display::fmt(eqn, f),
		}
	}
}
impl From<String> for Value<'_> {
	fn from(value: String) -> Self {
		Value::String(value)
	}
}
impl From<u32> for Value<'_> {
	fn from(value: u32) -> Self {
		Value::Integer(value)
	}
}
impl From<RealNumber> for Value<'_> {
	fn from(value: RealNumber) -> Self {
		Value::RealNumber(value)
	}
}
impl<'a> From<&'a Element> for Value<'a> {
	fn from(value: &'a Element) -> Self {
		Value::ElementRef(value)
	}
}
impl<'a> From<MolecularFormula<'a>> for Value<'a> {
	fn from(value: MolecularFormula<'a>) -> Self {
		Value::MolecularFormula(value)
	}
}
impl<'a> From<ChemEqn<'a>> for Value<'a> {
	fn from(value: ChemEqn<'a>) -> Self {
		Value::ChemEqn(value)
	}
}

trait ExpectFromValue<'a>: Sized {
	fn expect_from_value(value: Value<'a>) -> Result<Self, BadTypeError<'a>>;
}
impl<'a> ExpectFromValue<'a> for String {
	fn expect_from_value(value: Value<'a>) -> Result<Self, BadTypeError<'a>> {
		if let Value::String(string) = value {
			Ok(string)
		} else {
			Err(BadTypeError { expected_type: DataType::String, found_value: value })
		}
	}
}
impl<'a> ExpectFromValue<'a> for u32 {
	fn expect_from_value(value: Value<'a>) -> Result<Self, BadTypeError<'a>> {
		if let Value::Integer(integer) = value {
			Ok(integer)
		} else {
			Err(BadTypeError { expected_type: DataType::Integer, found_value: value })
		}
	}
}
impl<'a> ExpectFromValue<'a> for RealNumber {
	fn expect_from_value(value: Value<'a>) -> Result<Self, BadTypeError<'a>> {
		if let Value::RealNumber(real) = value {
			Ok(real)
		} else {
			Err(BadTypeError { expected_type: DataType::RealNumber, found_value: value })
		}
	}
}
impl<'a> ExpectFromValue<'a> for &'a Element {
	fn expect_from_value(value: Value<'a>) -> Result<Self, BadTypeError<'a>> {
		if let Value::ElementRef(element) = value {
			Ok(element)
		} else {
			Err(BadTypeError { expected_type: DataType::ElementRef, found_value: value })
		}
	}
}
impl<'a> ExpectFromValue<'a> for MolecularFormula<'a> {
	fn expect_from_value(value: Value<'a>) -> Result<Self, BadTypeError<'a>> {
		if let Value::MolecularFormula(formula) = value {
			Ok(formula)
		} else {
			Err(BadTypeError { expected_type: DataType::MolecularFormula, found_value: value })
		}
	}
}
impl<'a> ExpectFromValue<'a> for ChemEqn<'a> {
	fn expect_from_value(value: Value<'a>) -> Result<Self, BadTypeError<'a>> {
		if let Value::ChemEqn(eqn) = value {
			Ok(eqn)
		} else {
			Err(BadTypeError { expected_type: DataType::ChemEqn, found_value: value })
		}
	}
}

#[derive(Debug)]
pub enum DictAccessError<'a> {
	BadType(BadTypeError<'a>),
	UndefinedId(UndefinedIdentifierError<'a>),
}
impl Display for DictAccessError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Dictionary access error: ")?;
		match self {
			DictAccessError::BadType(err) => Display::fmt(err, f),
			DictAccessError::UndefinedId(err) => Display::fmt(err, f),
		}
	}
}
impl Error for DictAccessError<'_> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			DictAccessError::BadType(err) => Some(err),
			DictAccessError::UndefinedId(err) => Some(err),
		}
	}
}

#[derive(Debug)]
pub struct BadTypeError<'a> {
	expected_type: DataType,
	found_value: Value<'a>,
}
impl Display for BadTypeError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Expected type {}, found value {}", self.expected_type, self.found_value)
	}
}
impl Error for BadTypeError<'_> {}
impl<'a> From<BadTypeError<'a>> for DictAccessError<'a> {
	fn from(value: BadTypeError<'a>) -> Self {
		DictAccessError::BadType(value)
	}
}

#[derive(Debug)]
pub struct UndefinedIdentifierError<'a> {
	name: &'a str,
}
impl Display for UndefinedIdentifierError<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Undefined identifier '{}'", self.name)
	}
}
impl Error for UndefinedIdentifierError<'_> {}
impl From<UndefinedIdentifierError<'_>> for DictAccessError<'_> {
	fn from(value: UndefinedIdentifierError<'_>) -> Self {
		DictAccessError::UndefinedId(value)
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

	pub fn clear_value(&mut self, name: &str) -> Option<Value> {
		self.values.remove(name)
	}

	pub fn assign_value<T: Into<Value<'a>>>(&mut self, name: &str, value: T) {
		self.values.insert(name.to_owned(), value.into());
	}

	pub fn get_value<'b>(&self, name: &'b str) -> Result<&Value<'a>, UndefinedIdentifierError<'b>> {
		self.values.get(name).ok_or(UndefinedIdentifierError { name })
	}

	pub fn load_elements(&mut self, p_table: &'a PeriodicTable) {
		for element_info in p_table {
			self.values
				.insert(element_info.symbol.clone(), Value::ElementRef(element_info));
		}
	}
}
