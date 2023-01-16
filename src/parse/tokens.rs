use crate::helper::peek_iter::PeekIter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
	Unknown(String),
	Whitespace,
	Identifier(String),
	StringLiteral(String),
	Integer(String),
	Real(String),
	LParen,
	RParen,
	LBrack,
	RBrack,
	LCurly,
	RCurly,
	Dot,
	Bang,
	Cash,
	CashCash,
	EqualSign,
	PlusSign,
	MinusSign,
	MulSign,
	DivSign,
	PowSign,
	Comma,
	Colon,
	Arrow,
	Ellipse,
}

type StrTokPair = (&'static str, Token);

static TOKEN_STRINGS: &[StrTokPair] = &[
	("(", Token::LParen),
	(")", Token::RParen),
	("[", Token::LBrack),
	("]", Token::RBrack),
	("{", Token::LCurly),
	("}", Token::RCurly),
	(".", Token::Dot),
	("!", Token::Bang),
	("$", Token::Cash),
	("$$", Token::CashCash),
	("=", Token::EqualSign),
	("+", Token::PlusSign),
	("-", Token::MinusSign),
	("*", Token::MulSign),
	("/", Token::DivSign),
	("^", Token::PowSign),
	(",", Token::Comma),
	(":", Token::Colon),
	("->", Token::Arrow),
	("...", Token::Ellipse),
];

/// An iterator adaptor on an Iterator<Item = char> that tokenizes the items
pub struct Tokens<I: Iterator<Item = char>> {
	source: PeekIter<I>,
	token_strings: &'static [StrTokPair],
}

impl<I: Iterator<Item = char>> Tokens<I> {
	pub fn new(source: I) -> Self {
		Tokens {
			source: PeekIter::new(source),
			token_strings: &TOKEN_STRINGS,
		}
	}

	fn check_match(&mut self, pattern: &str) -> bool {
		for (i, pattern_char) in pattern.chars().enumerate() {
			match self.source.peek(i) {
				None => return false,
				Some(stream_char) => {
					if *stream_char != pattern_char {
						return false;
					}
				},
			}
		}
		true
	}

	/// Returns the longest possible token from the next characters in the stream and removes those
	/// characters. If no token matched, returns None and no characters are removed.
	fn get_longest_simple_token(&mut self) -> Option<Token> {
		let (longest_matching_pattern, token) = self
			.token_strings
			.iter()
			.filter(|(pattern, _)| self.check_match(pattern))
			.max_by(|(str_a, _), (str_b, _)| str_a.cmp(str_b))?;
		for _ in 0..longest_matching_pattern.len() {
			self.source.next();
		}
		Some(token.clone())
	}

	/// Returns the longest possible String from the next characters in the stream and removes those
	/// characters. Uses the function valid_char to determine whether a character is a valid
	/// character for the string. Returns an empty String if no characters constitute a valid token.
	fn get_longest_valid_string(&mut self, mut is_valid_char: impl FnMut(&char) -> bool) -> String {
		let mut result = String::new();
		while let Some(next_char) = self.source.next_if(&mut is_valid_char) {
			result.push(next_char);
		}
		result
	}
}

impl<I: Iterator<Item = char>> Iterator for Tokens<I> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		// underscores are not part of the tokenization
		while self.source.next_if(|c| *c == '_').is_some() {}

		// check the token type by peeking the next character
		let peek_char = self.source.peek(0)?;
		if peek_char.is_ascii_whitespace() {
			self.get_longest_valid_string(char::is_ascii_whitespace);
			Some(Token::Whitespace)
		} else if peek_char.is_ascii_alphabetic() {
			let mut is_first_char = true;
			Some(Token::Identifier(self.get_longest_valid_string(|c| {
				if is_first_char {
					is_first_char = false;
					return true; // already checked that the first character is valid
				}
				c.is_ascii_lowercase()
			})))
		} else if *peek_char == '\'' {
			self.source.next(); // discard the apostrophe
			Some(Token::Identifier(self.get_longest_valid_string(|c| {
				c.is_ascii_alphabetic() || c.is_ascii_digit()
			})))
		} else if *peek_char == '\"' {
			self.source.next(); // discard the opening quotation
			let string = self.get_longest_valid_string(|c| *c != '\"');
			self.source.next(); // discard the closing quotation
			Some(Token::StringLiteral(string))
		} else if peek_char.is_ascii_digit() {
			let mut seen_decimal = false;
			let mut seen_exp = false;
			let number_string = self.get_longest_valid_string(|c| {
				if c.is_ascii_digit() {
					true
				} else if !seen_decimal && *c == '.' {
					seen_decimal = true;
					true
				} else if seen_decimal && !seen_exp && *c == 'e' {
					seen_exp = true;
					true
				} else {
					false
				}
			});

			if seen_decimal {
				Some(Token::Real(number_string))
			} else {
				Some(Token::Integer(number_string))
			}
		} else {
			match self.get_longest_simple_token() {
				None => Some(Token::Unknown(
					self.source
						.next()
						.expect("Another char should've existed")
						.to_string(),
				)),
				some_token => some_token,
			}
		}
	}
}

pub trait IntoTokenIter {
	type SourceIter: Iterator<Item = char>;

	fn into_token_iter(self) -> Tokens<Self::SourceIter>;
}

impl<I: Iterator<Item = char>> IntoTokenIter for I {
	type SourceIter = I;

	fn into_token_iter(self) -> Tokens<I> {
		Tokens::new(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Token::*;

	#[test]
	fn tokenizes_properly() {
		let input = "'notregu1ar_idEnt1-fier*=-->(< ....caLiFor_ni-aGur!$$$123 .56.4e2 1.234.a ? ";
		let tokens_are: Vec<Token> = input.chars().into_token_iter().collect();
		let tokens_should_be = vec![
			Identifier("notregu1ar".to_string()),
			Identifier("id".to_string()),
			Identifier("Ent".to_string()),
			Integer("1".to_string()),
			MinusSign,
			Identifier("fier".to_string()),
			MulSign,
			EqualSign,
			MinusSign,
			Arrow,
			LParen,
			Unknown("<".to_string()),
			Whitespace,
			Ellipse,
			Dot,
			Identifier("ca".to_string()),
			Identifier("Li".to_string()),
			Identifier("For".to_string()),
			Identifier("ni".to_string()),
			MinusSign,
			Identifier("a".to_string()),
			Identifier("Gur".to_string()),
			Bang,
			CashCash,
			Cash,
			Integer("123".to_string()),
			Whitespace,
			Dot,
			Real("56.4e2".to_string()),
			Whitespace,
			Real("1.234".to_string()),
			Dot,
			Identifier("a".to_string()),
			Whitespace,
			Unknown("?".to_string()),
			Whitespace,
		];

		assert_eq!(tokens_are, tokens_should_be);
	}
}
