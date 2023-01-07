use chem_data::elements::*;
use std::fs;

pub mod cmd_interface;

pub mod chem_data;

pub fn do_something() {
	let p_table_file_path = "ptable.txt";
	let p_table = fs::read_to_string(p_table_file_path).expect("Should have been able to read the file.");
	let p_table = PeriodicTable::from(p_table);
	println!("Read the following from {p_table_file_path}:\n{}", p_table);

	let h = p_table.get_element("H").unwrap();
	println!("Hydrogen is {}", h.identity);
}