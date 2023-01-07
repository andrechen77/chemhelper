pub enum Token {
	Unknown(String),
	Symbol {symbol: String},
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

mod tests {
	use super::*;
}
