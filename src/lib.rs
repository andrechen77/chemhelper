use crate::{
	// chem_data::{dictionary::Dictionary, elements::PeriodicTable},
	cmd_interface::UserInputIter,
	parse::{expression::parse_str, tokens::IntoTokenIter},
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

	let _user_lines = UserInputIter::new("\nEnter formula");

	for line in _file_lines {
		println!("You entered: {}", line);
		println!("\nTokenized as: =============================");
		for token in line.chars().into_token_iter() {
			print!("{:?}, ", token);
		}
		println!("");
		match parse_str(&line) {
			Ok(expr) => {
				println!("\nParsed as =============================");
				println!("{:?}", expr);
			},
			Err(error) => {
				println!("Encountered error: {:?}", error);
			},
		}
	}
}
