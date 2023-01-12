use super::parser::{ParseError, Parser};
use crate::chem_data::{dictionary::Dictionary, elements::Element};

pub trait FromStrWithDict<'a>: Sized {
	fn from_str_w_dict(dict: &'a Dictionary, string: &str) -> Result<Self, ParseError<'a>>;
}

impl<'a> FromStrWithDict<'a> for Element<'a> {
	fn from_str_w_dict(dict: &'a Dictionary, string: &str) -> Result<Self, ParseError<'a>> {
		Parser::from(string).expect_element(dict)
	}
}
