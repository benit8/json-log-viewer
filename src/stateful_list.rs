use tui::widgets::ListState;

pub struct StatefulList<T> {
	state: ListState,
	items: Vec<T>,
}

impl<T> StatefulList<T> {
	pub fn with_items(items: Vec<T>) -> StatefulList<T> {
		let mut list = StatefulList {
			state: ListState::default(),
			items,
		};
		if list.items.len() > 0 {
			list.state.select(Some(0));
		}
		list
	}

	pub fn state_mut(&mut self) -> &mut ListState {
		&mut self.state
	}

	pub fn items(&self) -> &Vec<T> {
		&self.items
	}

	pub fn add(&mut self, item: T) {
		self.items.push(item);
	}

	pub fn select_next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i >= self.items.len() - 1 {
					i
				} else {
					i + 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn select_previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					i
				} else {
					i - 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn unselect(&mut self) {
		self.state.select(None);
	}
}
