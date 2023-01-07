use std::fmt;

#[derive(Debug)]
pub struct PeriodicTable {
	elements: Vec<ElementInfo>
}

impl PeriodicTable {
	pub fn from(string: String) -> Self {
		PeriodicTable {
			elements: string.lines().map(|line| {
				let mut fields = line.split(' ');
				ElementInfo {
					atomic_number: fields.next()
						.expect("There should've been an atomic number as the first item of the line")
						.parse()
						.expect("Should've been able to parse the atomic number as an integer"),
					name: fields.next()
						.expect("There should've been an element name as the second item of the line").to_string(),
					symbol: fields.next().expect("There should've been an atomic as the third item of the line").to_string(),
				}
			}).collect()
		}
	}

	pub fn add_element(&mut self, element: ElementInfo) {
		self.elements.push(element);
	}
}

impl fmt::Display for PeriodicTable {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for element in &self.elements {
			write!(f, "{}\n", element)?;
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct ElementInfo {
	pub atomic_number: i32,
	pub name: String,
	pub symbol: String,
}

impl fmt::Display for ElementInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}\t{}\t| {}]", self.atomic_number, self.name, self.symbol)
	}
}