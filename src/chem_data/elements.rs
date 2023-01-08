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
				let atomic_number = fields.next()
					.expect("There should've been an atomic number as the first item of the line")
					.parse()
					.expect("Should've been able to parse the atomic number as an integer");
				let symbol = fields.next()
					.expect("There should've been an atomic as the third item of the line")
					.to_string();
				let name = fields.next()
					.expect("There should've been an element name as the second item of the line")
					.to_string();
				ElementInfo {atomic_number, symbol, name}
			}).collect()
		}
	}

	pub fn add_element(&mut self, element: ElementInfo) {
		self.elements.push(element);
	}

	pub fn get_element(&self, symbol: &str) -> Option<Element> {
		Some(Element {identity: self.elements
			.iter()
			.find(|e| e.symbol == symbol)?
		})
	}
}

impl fmt::Display for PeriodicTable {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for element in &self.elements {
			writeln!(f, "{}", element)?;
		}
		Ok(())
	}
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ElementInfo {
	pub atomic_number: i32,
	pub symbol: String,
	pub name: String,
}

impl fmt::Display for ElementInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{} {} | {}]", self.atomic_number, self.symbol, self.name)
	}
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Element<'a> {
	pub identity: &'a ElementInfo,
}

impl fmt::Display for Element<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.identity.symbol)
	}
}