use crate::{
	chem_data::{
		// formulas::MolecularFormula,
		dictionary::Dictionary,
		elements::PeriodicTable,
	},
	cmd_interface::get_user_input,
	parse::parser::Parser,
	parse::tokens::IntoTokenIter,
};

pub mod chem_data;
pub mod cmd_interface;
pub mod helper;
pub mod parse;

pub fn do_something() {
	let p_table = PeriodicTable::from(std::fs::read_to_string("ptable.txt").unwrap());
	let mut dict = Dictionary::new();
	dict.load_elements(&p_table);

	let mut user_input: String;
	loop {
		user_input = loop {
			match get_user_input("Enter something") {
				Ok(user_input) => break user_input,
				Err(_) => continue,
			};
		};
		println!("You entered {}", user_input);
		if user_input == "stop" {
			break;
		}

		let tokens = user_input.chars().into_token_iter();
		let mut parser = Parser::new(tokens);
		while let Ok(element) = parser.expect_element(&dict) {
			println!("Found element: {}", element);
		}
		println!("Unparseable tokens:");
		while let Some(token) = parser.get_raw_token() {
			println!("{:?}", token);
		}
	}
}
