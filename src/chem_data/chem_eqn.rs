use std::fmt::Display;

use super::formulas::MolecularFormula;
use crate::helper::coeff_vec::CoeffVec;
pub use crate::helper::coeff_vec::Num;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ChemEqn<'a> {
	// positive coefficients are products, negative coefficients are reactants
	specieses: CoeffVec<MolecularFormula<'a>>,
}

impl<'a> ChemEqn<'a> {
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

impl Display for ChemEqn<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!();
	}
}
