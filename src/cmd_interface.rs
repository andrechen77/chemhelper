use std::io;

pub fn get_user_input(prompt: &str) -> io::Result<String> {
	println!("{prompt}:");
	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input)?;
	Ok(user_input.trim().to_string())
}