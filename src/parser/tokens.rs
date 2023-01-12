use crate::helper::peek_iter::PeekIter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Unknown(String),
	Whitespace,
    Identifier(String),
    Integer(u32),
	Real(String),
    LParen,
    RParen,
	LBrack,
	RBrack,
	LCurly,
	RCurly,
	Colon,
	EqualSign,
	Comma,
    PlusSign,
	MinusSign,
	MulSign,
	DivSign,
	PowSign,
	Dot,
    Arrow,
	Ellipse,
}

type StrTokPair = (&'static str, Token);

pub static TOKEN_STRINGS: &[StrTokPair] = &[
	("(", Token::LParen),
	(")", Token::RParen),
	("[", Token::LBrack),
	("]", Token::RBrack),
	("{", Token::LCurly),
	("}", Token::RCurly),
	(":", Token::Colon),
	("=", Token::EqualSign),
	(",", Token::Comma),
	("+", Token::PlusSign),
	("-", Token::MinusSign),
	("*", Token::MulSign),
	("/", Token::DivSign),
	("^", Token::PowSign),
	(".", Token::Dot),
	("->", Token::Arrow),
	("``", Token::Ellipse),
];

/// An iterator adaptor on an Iterator<Item = char> that tokenizes the items
pub struct Tokens<'a, I: Iterator<Item = char>> {
	source: PeekIter<I>,
	token_strings: &'a [StrTokPair],
}

impl<'a, I: Iterator<Item = char>> Tokens<'a, I> {
	pub fn new(source: I, token_strings: &'a[StrTokPair]) -> Self {
		Tokens {
			source: PeekIter::new(source),
			token_strings,
		}
	}

	fn check_match(&mut self, pattern: &str) -> bool {
		for (i, pattern_char) in pattern.chars().enumerate() {
			match self.source.peek(i) {
				None => return false,
				Some(stream_char) => if *stream_char != pattern_char {
					return false;
				}
			}
		}
		true
	}

	/// Returns the longest possible token from the next characters in the stream and removes those
	/// characters. If no token matched, returns None and no characters are removed.
	fn get_longest_simple_token(&mut self) -> Option<Token> {
		let (longest_matching_pattern, token) = self.token_strings.iter()
			.filter(|(pattern, _)| {
				self.check_match(pattern)
			})
			.max_by(|(str_a, _), (str_b, _)| {
				str_a.cmp(str_b)
			})?;
		for _ in 0..longest_matching_pattern.len() {
			self.source.next();
		}
		Some(token.clone())
	}

	/// Returns the longest possible String from the next characters in the stream and removes those
	/// characters. Uses the function valid_first to determine whether a character is a valid first
	/// character for the string, and valid_tail to determine whether a character is a valid
	/// non-first character for the string. Returns an empty String if no characters constitute a 
	/// valid token.
	fn get_longest_valid_string(
		&mut self,
		valid_first: impl FnOnce(&char) -> bool,
		mut valid_tail: impl FnMut(&char) -> bool,
	) -> String {
		let mut result = String::new();

		if let Some(first_char) = self.source.next_if(valid_first) {
			result.push(first_char);
			while let Some(next_char) = self.source.next_if(&mut valid_tail) {
				result.push(next_char);
			}
		}
		result
	}
}

impl<I: Iterator<Item = char>> Iterator for Tokens<'_, I> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let peek_char = self.source.peek(0)?;

		if peek_char.is_whitespace() {
			self.get_longest_valid_string(
				|_| true,
				char::is_ascii_whitespace,
			);
			Some(Token::Whitespace)
		} else if peek_char.is_ascii_alphabetic() || *peek_char == '_' {
			Some(Token::Identifier(self.get_longest_valid_string(
				|_| true, // already checked that the first character is valid
				|c| c.is_ascii_lowercase() || *c == '-',
			)))
		} else if *peek_char == '\'' {
			self.source.next(); // discard the apostrophe
			Some(Token::Identifier(self.get_longest_valid_string(
				|c| c.is_ascii_alphabetic() || *c == '_',
				|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || *c == '_' || *c == '-',
			)))
		} else if peek_char.is_ascii_digit() {
			Some(Token::Integer(self.get_longest_valid_string(
				char::is_ascii_digit,
				char::is_ascii_digit,
			).parse().expect("Should've been a parseable digits-only String")))
		} else if *peek_char == '#' {
			self.source.next(); // discard hashtag character
			let valid_real_char = |c: &char| {
				let seen_decimal = false;
				if c.is_ascii_digit() {
					return true;
				}
				if !seen_decimal && *c == '.' {
					return true;
				}
				false
			};
			Some(Token::Real(self.get_longest_valid_string(
				valid_real_char,
				valid_real_char
			)))
		} else {
			match self.get_longest_simple_token() {
				None => Some(Token::Unknown(self.source.next().expect("Another char should've existed").to_string())),
				some_token => some_token,
			}
		}
	}
}

pub trait IntoTokenIter {
	type SourceIter: Iterator<Item = char>;

	fn into_token_iter<'a>(self, token_strings: &'a [StrTokPair]) -> Tokens<'a, Self::SourceIter>;
}

impl<I: Iterator<Item = char>> IntoTokenIter for I {
	type SourceIter = I;

	fn into_token_iter<'a>(self, token_strings: &'a [StrTokPair]) -> Tokens<'a, I> {
		Tokens::new(self, token_strings)
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn tokenizes_properly() {
        let input = "'regular_1dEnti-fier*=-->(< caLiFor_ni-aGurls1234 .5678.4#1.234a ? ";
        let tokens_are: Vec<Token> = input.chars().into_token_iter(TOKEN_STRINGS).collect();
        let tokens_should_be = vec![
            Identifier("regular_1dEnti-fier".to_string()),
            MulSign,
            EqualSign,
            MinusSign,
            Arrow,
            LParen,
            Unknown("<".to_string()),
            Whitespace,
            Identifier("ca".to_string()),
            Identifier("Li".to_string()),
            Identifier("For".to_string()),
            Identifier("_ni-a".to_string()),
            Identifier("Gurls".to_string()),
            Integer(1234),
            Whitespace,
			Dot,
            Integer(5678),
            Dot,
            Integer(4),
            Real("1.234".to_string()),
            Identifier("a".to_string()),
            Whitespace,
            Unknown("?".to_string()),
            Whitespace,
        ];

        assert_eq!(tokens_are, tokens_should_be);
    }
}