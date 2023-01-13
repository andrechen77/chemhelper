use std::io;

pub fn get_user_input(prompt: &str) -> String {
	loop {
		println!("{prompt}");
		let mut buffer = String::new();
		match io::stdin().read_line(&mut buffer) {
			Ok(_) => break buffer.trim().to_string(),
			Err(_) => continue,
		}
	}
}

pub struct UserInputIter {
	prompt: String,
}

impl UserInputIter {
	pub fn new(prompt: &str) -> Self {
		UserInputIter {
			prompt: prompt.to_string(),
		}
	}
}

impl Iterator for UserInputIter {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let input = get_user_input(&self.prompt);
		if input == "stop" {
			None
		} else {
			Some(input)
		}
	}
}
