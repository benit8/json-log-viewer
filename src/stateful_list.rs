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

	pub fn unselect(&mut self) {
		self.state.select(None);
	}
}
