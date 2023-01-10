use std::ops::{Add, AddAssign, Mul, MulAssign};

pub type Num = i32;

/// A Vec that also has coefficients associated with its elements
#[derive(Debug, PartialEq, Default, Clone)]
pub struct CoeffVec<T: Clone + PartialEq> {
	pairs: Vec<(T, Num)>,
}

impl<T: Clone + PartialEq> CoeffVec<T> {
	pub fn new() -> Self {
		Self {pairs: Vec::new()}
	}

	pub fn set_coeff(&mut self, element: &T, new_coeff: Num) {
		if new_coeff == 0 {
			self.pairs.retain(|(e, _)| e != element);
			return;
		}

		if let Some((_, coeff)) = self.pairs.iter_mut().find(|(e, _)| e == element) {
			*coeff = new_coeff;
		} else {
			self.pairs.push((element.clone(), new_coeff));
		}
	}

	pub fn get_coeff(&self, element: &T) -> Num {
		match self.pairs.iter().find(|(e, _)| e == element) {
			Some((_, count)) => *count,
			None => 0,
		}
	}
}

impl<T: Clone + PartialEq> IntoIterator for CoeffVec<T> {
	type Item = (T, Num);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.pairs.into_iter()
	}
}

impl<'a, T: Clone + PartialEq> IntoIterator for &'a mut CoeffVec<T> {
	type Item = &'a mut (T, Num);
	type IntoIter = std::slice::IterMut<'a, (T, Num)>;

	fn into_iter(self) -> Self::IntoIter {
		self.pairs.iter_mut()
	}
}

impl<'a, T: Clone + PartialEq> IntoIterator for &'a CoeffVec<T> {
	type Item = &'a (T, Num);
	type IntoIter = std::slice::Iter<'a, (T, Num)>;

	fn into_iter(self) -> Self::IntoIter {
		self.pairs.iter()
	}
}

impl<T: Clone + PartialEq> AddAssign for CoeffVec<T> {
	fn add_assign(&mut self, rhs: Self) {
		for (element, coeff) in rhs.pairs {
			self.set_coeff(
				&element,
				self.get_coeff(&element) + coeff
			);
		}
	}
}

impl<T: Clone + PartialEq> Add for CoeffVec<T> {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T: Clone + PartialEq> MulAssign<Num> for CoeffVec<T> {
	fn mul_assign(&mut self, rhs: Num) {
		if rhs == 0 {
			self.pairs.clear();
			return;
		}

		for (_, coeff) in &mut self.pairs {
			*coeff *= rhs;
		}
	}
}

impl<T: Clone + PartialEq> Mul<Num> for CoeffVec<T> {
	type Output = Self;

	fn mul(mut self, rhs: Num) -> Self::Output {
		self *= rhs;
		self
	}
}