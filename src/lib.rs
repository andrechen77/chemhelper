use chem_data::parser::*;

pub mod cmd_interface;

pub mod chem_data;

pub fn do_something() {
	let input = read_from_file("input.txt");
	let mut token_iter = TokenIter::from_char_iter(input.chars());
	while let Some(next_token) = token_iter.next() {
		println!("found: {:?}", next_token);
	}
}

fn read_from_file(file_path: &str) -> String {
	std::fs::read_to_string(file_path).expect("Should've been able to read the file")
}