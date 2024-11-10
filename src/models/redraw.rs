use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Redraw {
	pub entries: bool,
	pub peak_volume: Option<usize>,
	pub resize: bool,
	pub affected_entries: HashSet<usize>,
	pub context_menu: bool,
}

impl Redraw {
	pub fn reset(&mut self) {
		*self = Redraw::default();
	}
	pub fn anything(&self) -> bool {
		self.entries
			|| self.peak_volume.is_some()
			|| self.resize
			|| self.context_menu
			|| !self.affected_entries.is_empty()
	}
}
