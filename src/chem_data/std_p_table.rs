use super::elements::PeriodicTable;

static mut MAYBE_P_TABLE: Option<Box<PeriodicTable>> = None;

pub fn std_p_table() -> &'static PeriodicTable {
	// This is disgusting and there must be some library way to do this
	unsafe {
		if let Some(p_table_box) = &MAYBE_P_TABLE {
			return p_table_box.as_ref()
		}

		MAYBE_P_TABLE = Some(Box::new(PeriodicTable::from(
			std::fs::read_to_string("ptable.txt")
				.unwrap()
		)));
		MAYBE_P_TABLE
			.as_ref()
			.expect("Should've just been initialized")
			.as_ref()
	}
}