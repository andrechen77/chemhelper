use std::fmt::Display;

/// A real number with significant figures.
#[derive(Debug, PartialEq, Clone)]
pub struct RealNumber {}
impl Display for RealNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!();
	}
}
