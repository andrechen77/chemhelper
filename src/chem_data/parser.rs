#[derive(Debug, PartialEq)]
pub enum Token {
	Unknown(String),
	Symbol(String),
	Number(i32),
	LeftParen,
	RightParen,
}

pub struct TokenIter<'a> {
	chars: Option<Box<dyn Iterator<Item = char> + 'a>>,
}

impl<'a> TokenIter<'a> {
	pub fn from_char_iter<I>(chars: I) -> Self
	where I: Iterator<Item = char> + 'a {
		TokenIter {chars: Some(Box::new(chars))}
	}
}

impl<'a> Iterator for TokenIter<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		// filter out possibility of no char iterator
		if let None = self.chars {
			return None;
		}

		// get the next non-whitespace character
		let mut chars = self.chars
			.take()
			.expect("Possibility of None should've been filtered out above")
			.skip_while(|next_char| next_char.is_whitespace());
		let next_char = chars.next()?;
		self.chars = Some(Box::new(chars));

		let mut string = String::new();
		string.push(next_char);
		Some(Token::Unknown(string))
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
