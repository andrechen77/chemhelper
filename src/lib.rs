use crate::{
	// chem_data::{dictionary::Dictionary, elements::PeriodicTable},
	cmd_interface::UserInputIter,
	parse::{expression::parser, tokens::IntoTokenIter},
};

pub mod chem_data;
pub mod cmd_interface;
pub mod helper;
pub mod parse;

pub fn do_something() {
	// let p_table = PeriodicTable::from(std::fs::read_to_string("ptable.txt").unwrap());
	// println!("Loaded elements:\n{}", p_table);
	// let mut dict = Dictionary::new();
	// dict.load_elements(&p_table);
	// println!("Dictionary:\n{:#?}", dict);

	let file_contents = std::fs::read_to_string("input.txt").unwrap();
	let _file_lines = file_contents.lines();

	let _user_lines = UserInputIter::new("Enter formula");

	for line in _user_lines {
		println!("You entered {}", line);
		println!("Tokenized as:");
		for token in line.chars().into_token_iter() {
			println!("{:?}", token);
		}
		match parser::parse_tokens(line.chars().into_token_iter()) {
			Ok(expr) => {
				println!("Parsed as {:#?}", expr);
			},
			Err(error) => {
				println!("Encountered error: {:?}", error);
			},
		}
	}
}
