use tui::widgets::{ListItem, ListState};

pub struct StatefulList<'a> {
	state: ListState,
	items: Vec<ListItem<'a>>,
}

impl<'a> StatefulList<'a> {
	pub fn new() -> StatefulList<'a> {
		StatefulList {
			state: ListState::default(),
			items: vec!(),
		}
	}

	pub fn state(&self) -> &ListState {
		&self.state
	}

	pub fn state_mut(&mut self) -> &mut ListState {
		&mut self.state
	}

	pub fn items(&self) -> &Vec<ListItem> {
		&self.items
	}

	pub fn push_item(&mut self, item: ListItem<'a>) {
		self.items.push(item);

		if self.items.len() == 1 {
			self.state.select(Some(0));
		}
	}

	pub fn select_n_up(&mut self, n: usize) {
		let i = match self.state.selected() {
			Some(i) => {
				if n > i {
					0
				} else {
					i - n
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn select_n_down(&mut self, n: usize) {
		let i = match self.state.selected() {
			Some(i) => {
				if i + n > self.items.len() - 1 {
					self.items.len() - 1
				} else {
					i + n
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn select_first(&mut self) {
		self.state.select(Some(0));
	}

	pub fn select_last(&mut self) {
		self.state.select(Some(self.items.len() - 1));
	}
}
