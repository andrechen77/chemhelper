use super::*;
use crate::parse::tokens::{IntoTokenIter, Token, Tokens};

pub fn parse_str(string: &str) -> Result<Box<dyn Expression>, ParseError> {
	parse_tokens(string.chars().into_token_iter())
}

pub fn parse_tokens<I: Iterator<Item = char>>(
	token_iter: Tokens<I>,
) -> Result<Box<dyn Expression>, ParseError> {
	let mut result = Box::new(WrapperExprBuilder::new());
	for token in token_iter {
		match result.add_token(token)? {
			None => (),
			Some(rejected) => return Err(ParseError::UnexpectedToken(rejected)),
		}
	}
	Ok(result.finish()?)
}

#[derive(Debug)]
pub enum ParseError {
	NoTokens,
	UnexpectedToken(Token),
	ExpectedTokens,
	RequiredParsingTimeIdentifier,
}

fn create_new_expression(first_token: Token) -> Result<Box<dyn ExpressionBuilder>, ParseError> {
	match first_token {
		Token::Identifier(name) => Ok(Box::new(IdentifierExprBuilder::new(name))),
		Token::StringLiteral(content) => Ok(Box::new(StringLiteralExprBuilder::new(content))),
		Token::Integer(value) => Ok(Box::new(IntegerLiteralExprBuilder::new(
			value
				.parse()
				.expect("Should've been a parseable digits-only string"),
		))),
		Token::Real(value) => Ok(Box::new(RealLiteralExprBuilder::new(value))),
		Token::LParen => Ok(Box::new(TupleExprBuilder::new())),
		Token::Cash => Ok(Box::new(MolecularFormulaExprBuilder::new())),
		Token::CashCash => Ok(Box::new(CondensedFormulaExprBuilder::new())),
		_ => Err(ParseError::UnexpectedToken(first_token)),
	}
}

/// An object that can be used to build expressions token by token. The add_token method attempts to
/// incorporate the specified token into the expression. If the expression being built is a compound
/// expression (i.e. composed of other expressions), there will typically be an "active expression"
/// that an ExpressionBuilder will pass tokens into. This happens recursively until the token
/// reaches the innermost active expression, at which point that innermost expression will attempt
/// to incorporate the token into itself. The add_token method will "reject" the token if it cannot
/// be incorporated into the expression AND the expression is in a valid state to be permanently
/// closed. If the token is "rejected" by the inner expression, the expression containing it should
/// receive the rejected token and attempt to incorporate it into itself (or if that is not possible,
/// attempt an infix operation restructuring). This upward propagation continues until some
/// ExpressionBuilder accepts the token or the token is rejected from the outermost
/// ExpressionBuilder, or the token cannot be rejected because some expression is not yet in a valid
/// state (at which point a ParseError is propagated). Any one ExpressionBuilder should never be
/// given Tokens again after it has rejected one (and thereby "closed" itself); to do otherwise is a
/// logic error.
trait ExpressionBuilder {
	/// Attempts to incorporate the specified token into the current ExpressionBuilder. If
	/// incorporation is unsuccessful, the token will either be rejected (if the current
	/// ExpressionBuilder is in a valid state) or a ParseError will be returned.
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError>;

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError>;

	fn parsing_time_identifier(&self) -> Result<&str, ParseError> {
		Err(ParseError::RequiredParsingTimeIdentifier)
	}
}

struct WrapperExprBuilder {
	maybe_inner_expr: Option<Box<dyn ExpressionBuilder>>,
}

impl WrapperExprBuilder {
	fn new() -> Self {
		Self {
			maybe_inner_expr: None,
		}
	}
}

impl ExpressionBuilder for WrapperExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		match &mut self.maybe_inner_expr {
			None => {
				if !matches!(token, Token::Whitespace) {
					self.maybe_inner_expr = Some(create_new_expression(token)?);
				}
			},
			Some(expr) => match expr.add_token(token)? {
				None => (),
				Some(Token::Whitespace) => (),
				Some(rejected) => {
					let (expr, maybe_rejected) = wrap_in_infix(
						self.maybe_inner_expr
							.take()
							.expect("There should've been an inner expr"),
						rejected,
					);
					self.maybe_inner_expr = Some(expr);
					if let Some(bad_token) = maybe_rejected {
						// WrapperExprBuilder is in a valid state if its inner_expr is
						// valid. Therefore, reject the token and close.
						return Ok(Some(bad_token));
					}
				},
			},
		}
		Ok(None)
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		self.maybe_inner_expr.ok_or(ParseError::NoTokens)?.finish()
	}
}

/// Represents an expression being built that currently only has an identifier, e.g.
/// `identifier`
struct IdentifierExprBuilder {
	identifier: String,
}

impl IdentifierExprBuilder {
	fn new(identifier: String) -> Self {
		Self { identifier }
	}
}

impl ExpressionBuilder for IdentifierExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		// always closed
		Ok(Some(token))
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		Ok(Box::new(Identifier {
			name: self.identifier,
		}))
	}

	fn parsing_time_identifier(&self) -> Result<&str, ParseError> {
		Ok(&self.identifier)
	}
}

/// Represents an expression being build that currently only has a string literal, e.g.
/// `"string literal here"`
struct StringLiteralExprBuilder {
	content: String,
}

impl StringLiteralExprBuilder {
	fn new(content: String) -> Self {
		Self { content }
	}
}

impl ExpressionBuilder for StringLiteralExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		// always closed
		Ok(Some(token))
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		Ok(Box::new(StringLiteral {
			content: self.content,
		}))
	}
}

/// Represents an expression being built that currently only has an integer literal, e.g.
/// `32`
struct IntegerLiteralExprBuilder {
	value: u32,
}

impl IntegerLiteralExprBuilder {
	fn new(value: u32) -> Self {
		Self { value }
	}
}

impl ExpressionBuilder for IntegerLiteralExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		// always closed
		Ok(Some(token))
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		Ok(Box::new(IntegerLiteral { value: self.value }))
	}
}

/// Represents an expression being built that currently only has a real literal, e.g.
/// `1.23e4`
struct RealLiteralExprBuilder {
	value: String,
}

impl RealLiteralExprBuilder {
	fn new(value: String) -> Self {
		Self { value }
	}
}

impl ExpressionBuilder for RealLiteralExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		Ok(Some(token))
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		Ok(Box::new(RealLiteral { value: self.value }))
	}
}

/// Represents an expression being built which is a tuple of other expressions, e.g.
/// `(`
/// `(a`
/// `(a,`
/// `(a, b`
/// `(a, b)`
/// The tuple may be open or closed
struct TupleExprBuilder {
	values: Vec<Box<dyn ExpressionBuilder>>,
	has_active_expr: bool,
	closed: bool,
}

impl TupleExprBuilder {
	fn new() -> Self {
		Self {
			values: Vec::new(),
			has_active_expr: false,
			closed: false,
		}
	}

	/// Attempts to add the token to the current active expression, or if none exists, to use the
	/// token to create a new expression that will become the current active expression. If the
	/// current active expression rejects the token, this method will return that; if the token is
	/// used to create an expression but this fails, this method propagates the error.
	fn add_to_active_expr(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		assert!(!self.closed);

		if self.has_active_expr {
			self.values
				.last_mut()
				.expect("There should've been an active expression")
				.add_token(token)
		} else {
			if !matches!(token, Token::Whitespace) {
				self.values.push(create_new_expression(token)?);
				self.has_active_expr = true;
			}
			Ok(None)
		}
	}
}

impl ExpressionBuilder for TupleExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		if self.closed {
			return Ok(Some(token));
		}

		match self.add_to_active_expr(token)? {
			None => (),
			Some(Token::Whitespace) => (),
			Some(Token::RParen) => self.closed = true,
			Some(Token::Comma) => self.has_active_expr = false,
			Some(rejected) => {
				// attempt to create an infix expression with the rejected token
				let (expr, maybe_rejected) = wrap_in_infix(
					self.values
						.pop()
						.expect("There should've been an active subexpression"),
					rejected,
				);
				self.values.push(expr);
				if let Some(rejected) = maybe_rejected {
					return Err(ParseError::UnexpectedToken(rejected));
				}
			},
		}
		Ok(None)
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		let mut values = Vec::new();
		for expr_builder in self.values.into_iter() {
			values.push(expr_builder.finish()?)
		}
		if !self.closed {
			return Err(ParseError::ExpectedTokens);
		}
		Ok(Box::new(TupleExpr { values }))
	}
}

struct SpecialSyntaxExprBuilder {
	seen_curlies: (bool, bool),
	// .0 is if the left one was seen, .1 is if the right one was seen
	// (false, true) is not a valid state
	inner_expr: Box<dyn ExpressionBuilder>,
}

impl SpecialSyntaxExprBuilder {
	fn new(syntax_name: &str) -> Self {
		Self {
			seen_curlies: (false, false),
			inner_expr: lookup_special_syntax_builder(syntax_name),
		}
	}
}

impl ExpressionBuilder for SpecialSyntaxExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		if self.seen_curlies.1 {
			return Ok(Some(token));
		}

		if !self.seen_curlies.0 {
			match token {
				Token::LCurly => {
					self.seen_curlies.0 = true;
					return Ok(None);
				},
				Token::Whitespace => return Ok(None),
				_ => return Err(ParseError::UnexpectedToken(token)),
			}
		}

		match self.inner_expr.add_token(token)? {
			None => (),
			Some(Token::Whitespace) => (),
			Some(Token::RCurly) => self.seen_curlies.1 = true,
			Some(rejected) => {
				// don't attempt to create an infix expression
				return Err(ParseError::UnexpectedToken(rejected));
			},
		}
		Ok(None)
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		if self.seen_curlies != (true, true) {
			return Err(ParseError::ExpectedTokens);
		}
		Ok(self.inner_expr.finish()?)
	}
}

fn lookup_special_syntax_builder(_syntax_name: &str) -> Box<dyn ExpressionBuilder> {
	todo!();
}

struct InfixOperationsExprBuilder {
	operands: Vec<Box<dyn ExpressionBuilder>>,
	operators: Vec<InfixOperator>,
}

impl InfixOperationsExprBuilder {
	fn new(first_operand: Box<dyn ExpressionBuilder>, first_operator: InfixOperator) -> Self {
		Self {
			operands: vec![first_operand],
			operators: vec![first_operator],
		}
	}

	/// Attempts to add the token to the current active expression, or if none exists, to use the
	/// token to create a new expression that will become the current active expression. If the
	/// current active expression rejects the token, this method will return that; if the token is
	/// used to create an expression but this fails, this method propagates the error.
	fn add_to_current_active_expr(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		if self.operands.len() == self.operators.len() {
			// the last item added was an operator
			if !matches!(token, Token::Whitespace) {
				self.operands.push(create_new_expression(token)?);
			}
			Ok(None)
		} else if self.operands.len() == self.operators.len() + 1 {
			// there is still an active operand
			self.operands
				.last_mut()
				.expect("There should've been an active operand")
				.add_token(token)
		} else {
			unreachable!()
		}
	}
}

impl ExpressionBuilder for InfixOperationsExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		match self.add_to_current_active_expr(token)? {
			None => (),
			Some(Token::Whitespace) => (),
			Some(rejected) => match InfixOperator::try_from(rejected) {
				Ok(operator) => self.operators.push(operator),
				Err(bad_token) => {
					// Since we must be in a valid state, reject the bad token and close this
					// InfixOperationsExprBuilder
					assert_eq!(self.operands.len() - self.operators.len(), 1);
					return Ok(Some(bad_token));
				},
			},
		}
		Ok(None)
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		// assert_eq!(self.operands.len() - self.operators.len(), 1); make this an Err() instead of panic
		if self.operands.len() - self.operators.len() != 1 {
			return Err(ParseError::ExpectedTokens);
		}
		let mut operands = Vec::new();
		for expr_builder in self.operands {
			operands.push(expr_builder.finish()?);
		}
		Ok(Box::new(InfixOperationsExpr {
			operands,
			operators: self.operators,
		}))
	}
}

fn wrap_in_infix(
	expr: Box<dyn ExpressionBuilder>,
	maybe_operator: Token,
) -> (Box<dyn ExpressionBuilder>, Option<Token>) {
	match InfixOperator::try_from(maybe_operator) {
		Ok(operator) => (
			Box::new(InfixOperationsExprBuilder::new(expr, operator)),
			None,
		),
		Err(bang @ Token::Bang) => (
			Box::new(SpecialSyntaxExprBuilder::new(
				match expr.parsing_time_identifier() {
					Ok(syntax_name) => syntax_name,
					Err(_) => return (expr, Some(bang)),
				},
			)),
			None,
		),
		Err(rejected) => (expr, Some(rejected)),
	}
}

impl TryFrom<Token> for InfixOperator {
	type Error = Token;

	/// Returns either the Token interpreted as an infix operator, or if this fails, returns the
	/// Token.
	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::Dot => Ok(InfixOperator::FunctionCall),
			Token::PlusSign => Ok(InfixOperator::Plus),
			Token::MinusSign => Ok(InfixOperator::Minus),
			Token::MulSign => Ok(InfixOperator::Mul),
			Token::DivSign => Ok(InfixOperator::Div),
			Token::PowSign => Ok(InfixOperator::Pow),
			unmatched => Err(unmatched),
		}
	}
}

struct MolecularFormulaExprBuilder {
	symbols_and_subscripts: Vec<Box<dyn ExpressionBuilder>>,
	charge: Option<(bool, Option<Box<dyn ExpressionBuilder>>)>,
	// true is positive, false is negative; None value means that part of the syntax hasn't been
	// encountered yet
	is_closed: bool,
}

impl MolecularFormulaExprBuilder {
	fn new() -> Self {
		Self {
			symbols_and_subscripts: Vec::new(),
			charge: None,
			is_closed: false,
		}
	}

	/// Attempts to add the specified token to the last symbol or subscript expression; if this
	/// gets rejected, then attempts to create a next symbol/subscript expression; if this gets
	/// rejected, returns the token
	fn add_to_symbol_or_subscr(&mut self, mut token: Token) -> Result<Option<Token>, ParseError> {
		assert!(self.charge.is_none());

		// Attempt to add it to the last symbol/subscr
		if let Some(active_expr) = self.symbols_and_subscripts.last_mut() {
			match active_expr.add_token(token)? {
				None => return Ok(None),
				Some(rejected) => token = rejected, // put the token back into the variable
			}
		}

		// Either there were no subexpressions or the last subexpression rejected
		// So try to create another expression using the token
		if !matches!(token, Token::Whitespace) {
			match create_new_expression(token) {
				Ok(expr) => {
					self.symbols_and_subscripts.push(expr);
					return Ok(None);
				},
				Err(ParseError::UnexpectedToken(rejected)) => token = rejected,
				_ => unreachable!("create_new_expression should never return anything else"),
			}
		}

		// Return the token
		Ok(Some(token))
	}
}

impl ExpressionBuilder for MolecularFormulaExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		if self.is_closed {
			return Ok(Some(token));
		}

		match &mut self.charge {
			None => {
				// no sign encountered yet
				if let Some(rejected) = self.add_to_symbol_or_subscr(token)? {
					match rejected {
						Token::PlusSign => self.charge = Some((true, None)),
						Token::MinusSign => self.charge = Some((false, None)),
						_ => {
							// all the subexpressions are valid so the formula must be valid too;
							// therefore reject and close
							self.is_closed = true;
							return Ok(Some(rejected));
						},
					}
				}
				Ok(None)
			},
			Some((_, magn_option @ None)) => {
				// a sign has been encountered but no magnitude expression
				*magn_option = Some(create_new_expression(token)?);
				// cannot be whitespace; once a sign is encountered, a magnitude expression is
				// immediately expected
				Ok(None)
			},
			Some((_, Some(magn))) => {
				// a magnitude expression exists
				// if the magntiude expression accepts, the token, so does the whole formula; if the
				// magnitude expression rejects, so does the whole formula
				match magn.add_token(token)? {
					Some(rejected) => {
						self.is_closed = true;
						Ok(Some(rejected))
					},
					None => Ok(None),
				}
			},
		}
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		// account the possibility for something like "$CO3+"
		let mut symbols_and_subscripts = Vec::new();
		for symbol_or_subscr in self.symbols_and_subscripts.into_iter() {
			symbols_and_subscripts.push(symbol_or_subscr.finish()?)
		}
		Ok(Box::new(MolecularFormulaExpr {
			symbols_and_subscripts,
			_charge: match self.charge {
				None => None,
				Some((sign, maybe_magn)) => Some((
					sign,
					maybe_magn.ok_or(ParseError::ExpectedTokens)?.finish()?,
				)),
			},
		}))
	}
}

struct CondensedFormulaExprBuilder {
	subformulas_and_subscripts: Vec<Box<dyn ExpressionBuilder>>,
	charge: Option<(bool, Option<Box<dyn ExpressionBuilder>>)>,
	// true is positive, false is negative; None value means that part of the syntax hasn't been
	// encountered yet
}

impl CondensedFormulaExprBuilder {
	fn new() -> Self {
		Self {
			subformulas_and_subscripts: Vec::new(),
			charge: None,
		}
	}
}

impl ExpressionBuilder for CondensedFormulaExprBuilder {
	fn add_token(&mut self, token: Token) -> Result<Option<Token>, ParseError> {
		todo!();
	}

	fn finish(self: Box<Self>) -> Result<Box<dyn Expression>, ParseError> {
		todo!();
	}
}

#[cfg(test)]
mod tests {}
