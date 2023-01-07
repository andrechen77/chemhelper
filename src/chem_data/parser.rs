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
		// cannot use skip_while method because we can't change the type of iterator
		while chars.next_if(|next_char| next_char.is_whitespace()).is_some() {}

		// check the first character
		let result = match chars.next()? {
			'(' => Token::LeftParen,
			')' => Token::RightParen,
			initial if initial.is_ascii_uppercase() =>
				Token::Symbol(format!(
					"{}{}",
					initial,
					next_chars_which(&mut chars, |c| c.is_ascii_lowercase())
				)),
			initial if initial.is_ascii_digit() =>
				Token::Number(
					format!("{}{}", initial, next_chars_which(&mut chars, |c| c.is_ascii_digit()))
						.parse()
						.expect("Should've only been given ascii digits so the parse should've passed")
				),
			other => Token::Unknown(other.to_string()),
		};

		// restore ownership of self.chars
		self.chars = Some(chars);

		// return the result
		Some(result)
	}
}

/// given a peekable char iterator, returns the next immediate characters which
/// satisfy the given predicate as a String
/// 
/// # Examples
/// ```
/// let sentence = "Hello there!".to_string();
/// let first_word = next_chars_which(sentence.chars(), |c| c.is_alphabetic());
/// assert_eq!(first_word, "Hello!");
/// ```
fn next_chars_which<P>(iter: &mut Peekable<impl Iterator<Item = char>>, mut predicate: P) -> String 
where
	P: FnMut(&char) -> bool
{
	// cannot use scan method because we can't change the type of iterator

	let mut result = String::new();
	while let Some(next_char) = iter.next_if(&mut predicate) {
		result.push(next_char);
	}
	result
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
