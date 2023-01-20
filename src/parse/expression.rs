use std::fmt::Debug;

use crate::chem_data::dictionary::{Dictionary, Value, DictAccessError, UndefinedIdentifierError, BadTypeError};
use crate::chem_data::elements::Element;
use crate::chem_data::formulas::MolecularFormula;

mod parser;

pub use parser::parse_str;
pub use parser::parse_tokens;
pub use parser::ParseError;

pub trait Expression {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>>;

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl Debug for dyn Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.forehead(f)
	}
}

pub enum EvaluationError<'a> {
	UndefinedIdentifier(UndefinedIdentifierError<'a>),
	BadType(BadTypeError<'a>),
}
impl<'a> From<UndefinedIdentifierError<'a>> for EvaluationError<'a> {
	fn from(value: UndefinedIdentifierError<'a>) -> Self {
		EvaluationError::UndefinedIdentifier(value)
	}
}
impl<'a> From<BadTypeError<'a>> for EvaluationError<'a> {
	fn from(value: BadTypeError<'a>) -> Self {
		EvaluationError::BadType(value)
	}
}

#[derive(Debug)]
pub struct Identifier {
	name: String,
}

impl Expression for Identifier {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		Ok(dict.get_value(&self.name).cloned()?)
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct StringLiteral {
	content: String,
}

impl Expression for StringLiteral {
	fn evaluate<'a>(self: Box<Self>, _dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		Ok(Value::String(self.content))
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct IntegerLiteral {
	value: u32,
}

impl Expression for IntegerLiteral {
	fn evaluate<'a>(self: Box<Self>, _dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		Ok(Value::Integer(self.value))
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct RealLiteral {
	value: String,
}

impl Expression for RealLiteral {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		todo!();
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct TupleExpr {
	values: Vec<Box<dyn Expression>>,
}

impl Expression for TupleExpr {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		todo!();
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct SpecialSyntaxExpr {
	syntax_name: Identifier,
	inner_expr: Box<dyn Expression>,
}

impl Expression for SpecialSyntaxExpr {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		todo!();
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct InfixOperationsExpr {
	operands: Vec<Box<dyn Expression>>,
	operators: Vec<InfixOperator>,
}

#[derive(Debug, Clone, Copy)]
enum InfixOperator {
	FunctionCall,
	Plus,
	Minus,
	Mul,
	Div,
	Pow,
}

impl Expression for InfixOperationsExpr {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		todo!();
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct MolecularFormulaExpr {
	symbols_and_subscripts: Vec<Box<dyn Expression>>,
	_charge: Option<(bool, Box<dyn Expression>)>,
	// true is positive, false is negative
}

impl Expression for MolecularFormulaExpr {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		let mut result = MolecularFormula::new();

		let mut values = Vec::new();
		for expr in self.symbols_and_subscripts.into_iter() {
			values.push(expr.evaluate(dict)?);
		}
		let mut values = values.into_iter().peekable();
		while let Some(value) = values.next() {
			let element: &Element = value.as_type::<&Element>()?;
			let subscript = match values.next_if(|val| matches!(val, Value::Integer(_))) {
				None => 1,
				Some(value) => value.as_type::<u32>().expect("Should've checked int type"),
			};
			result.set_subscr(element, result.get_subscr(element) + subscript);
		}
		Ok(Value::MolecularFormula(result))

		// TODO incorporate charges
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

#[derive(Debug)]
struct CondensedFormulaExpr {
	subformulas_and_subscripts: Vec<Box<dyn Expression>>,
	charge: Option<(bool, Box<dyn Expression>)>,
}

impl Expression for CondensedFormulaExpr {
	fn evaluate<'a>(self: Box<Self>, dict: &'a Dictionary) -> Result<Value<'a>, EvaluationError<'a>> {
		todo!();
	}

	fn forehead(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}
