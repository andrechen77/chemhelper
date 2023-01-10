use crate::{
	chem_data::{
		elements::PeriodicTable,
		formulas::MolecularFormula,
	},
	cmd_interface::get_user_input
};

pub mod cmd_interface;
pub mod chem_data;
pub mod parser;
pub mod helper;

pub fn do_something() {
	let p_table = PeriodicTable::from(read_from_file("ptable.txt"));

	let mut user_input: String;
	loop {
		user_input = loop {
			match get_user_input("Enter a formula") {
				Ok(user_input) => break user_input,
				Err(_) => continue,
			};
		};
		println!("You entered: {}", user_input);
		if user_input == "stop" {
			break;
		}
		let mut token_iter = parser::TokenIter::from_char_iter(user_input.chars()).peekable();
		let formula = MolecularFormula::from_token_iter(&p_table, &mut token_iter);
		println!("Formula parsed as: {}", formula);
	}
}

fn read_from_file(file_path: &str) -> String {
	std::fs::read_to_string(file_path).expect("Should've been able to read the file")
}