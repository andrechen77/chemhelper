use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
	Unknown(String),
	Symbol(String),
	Number(i32),
	LeftParen,
	RightParen,
}

pub struct TokenIter<I: Iterator<Item = char>> {
	chars: Option<Peekable<I>>,
}

impl<I: Iterator<Item = char>> TokenIter<I> {
	pub fn from_char_iter(chars: I) -> Self {
		TokenIter {chars: Some(chars.peekable())}
	}
}

impl<I: Iterator<Item = char>> Iterator for TokenIter<I> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		// take ownership of self.chars
		let mut chars = self.chars.take()?;

		// skip leading whitespace
		while chars.next_if(|next_char| next_char.is_whitespace()).is_some() {}

		// check the first character
		let result = match chars.next()? {
			'(' => Token::LeftParen,
			')' => Token::RightParen,
			initial if initial.is_alphabetic() => Token::Symbol(initial.to_string()),
			other => Token::Unknown(other.to_string()),
		};

		// restore ownership of self.chars
		self.chars = Some(chars);

		// return the result
		Some(result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Token::*;

	#[test]
	fn tokenizes_properly() {
		let formula = "     H2 Bee10 xpNe zt10 0 (No p)2  ?  ";
		let tokens_are = TokenIter::from_char_iter(formula.chars());
		let tokens_should_be = vec![
			Symbol("H".to_string()),
			Number(2),
			Symbol("Bee".to_string()),
			Number(10),
			Unknown("x".to_string()),
			Unknown("p".to_string()),
			Symbol("Ne".to_string()),
			Unknown("z".to_string()),
			Unknown("t".to_string()),
			Number(10),
			Number(0),
			LeftParen,
			Symbol("No".to_string()),
			Unknown("p".to_string()),
			RightParen,
			Number(2),
			Unknown("?".to_string()),
		];
		
		let mut zipped = tokens_are.zip(tokens_should_be.iter());
		while let Some((token_is, token_should_be)) = zipped.next() {
			assert_eq!(token_is, *token_should_be);
		}
	}
}
