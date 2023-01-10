use super::formulas::MolecularFormula;
use crate::helper::coeff_vec::CoeffVec;
pub use crate::helper::coeff_vec::Num;

#[derive(Default)]
pub struct Equation<'a> {
	// positive coefficients are products, negative coefficients are reactants
	specieses: CoeffVec<MolecularFormula<'a>>,
}

impl<'a> Equation<'a> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn set_coeff(&mut self, species: &'a MolecularFormula, new_coeff: Num) {
		self.specieses.set_coeff(species, new_coeff);
	}

	pub fn get_coeff(&mut self, species: &MolecularFormula) -> Num {
		self.specieses.get_coeff(species)
	}

	// TODO add solve(self) -> Self
}