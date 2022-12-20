use crate::stateful_list::StatefulList;

use async_std::channel::{Receiver, TryRecvError};
use crossterm::event::{self, Event, KeyCode};
use serde_json::{Map, Value};
use std::time::{Duration, Instant};
use tui::{
	backend::{Backend},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, Borders, List, ListItem},
	Frame, Terminal,
};


/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App<'a> {
	input_filename: String,
	log_receiver: &'a Receiver<Map<String, Value>>,
	list: StatefulList<Map<String, Value>>,
}

impl<'a> App<'a> {
	pub fn new(input: String, receiver: &'a Receiver<Map<String, Value>>) -> App<'a> {
		App {
			input_filename: input,
			log_receiver: receiver,
			list: StatefulList::with_items(vec![]),
		}
	}

	pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>, tick_rate: Duration) -> std::io::Result<()> {
		let mut last_tick = Instant::now();
		loop {
			terminal.draw(|f| self.ui(f))?;

			let timeout = tick_rate
				.checked_sub(last_tick.elapsed())
				.unwrap_or(Duration::from_secs(0));
			if crossterm::event::poll(timeout)? {
				if let Event::Key(key) = event::read()? {
					match key.code {
						KeyCode::Char('q') => return Ok(()),
						KeyCode::Down => self.list.select_n_down(1),
						KeyCode::End => self.list.select_last(),
						KeyCode::Home => self.list.select_first(),
						KeyCode::PageDown => self.list.select_n_down(terminal.size()?.height as usize),
						KeyCode::PageUp => self.list.select_n_up(terminal.size()?.height as usize),
						KeyCode::Up => self.list.select_n_up(1),
						_ => {}
					}
				}
			}

			if last_tick.elapsed() >= tick_rate {
				self.on_tick();
				last_tick = Instant::now();
			}
		}
	}

	fn on_tick(&mut self) -> Result<(), TryRecvError> {
		loop {
			match self.log_receiver.try_recv() {
				Ok(log) => self.list.add(log),
				Err(_) => break,
			}
		}

		Ok(())
	}

	fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
		let items: Vec<ListItem> = self.list
			.items()
			.iter()
			.map(|log| {
				let level_name = log["level_name"].as_str().unwrap_or("[NONE]");
				let datetime = log["datetime"].as_str().unwrap_or("--------------------------------");
				let message = log["message"].as_str().unwrap_or_default();
				// FIXME: assign an empty map
				let context = log["context"].as_object().unwrap();

				// Colorcode the level depending on its type
				let level_style = match level_name {
					"DEBUG" => Style::default().fg(Color::Magenta),
					"INFO" => Style::default().fg(Color::Blue),
					"NOTICE" => Style::default().fg(Color::Cyan),
					"WARNING" => Style::default().fg(Color::Yellow),
					"ERROR" | "CRITICAL" => Style::default().fg(Color::Red),
					_ => Style::default(),
				};

				let flattened_context = context.iter()
					.map(|(k, v)| format!("{} = {}", k, v.to_string()))
					.collect::<Vec<String>>()
					.join(", ");

				ListItem::new(Spans::from(vec![
					Span::raw(datetime.to_string()),
					Span::raw(" "),
					Span::styled(format!("{:<9}", level_name), level_style),
					Span::raw(" "),
					Span::raw(format!("{:<32}", message)),
					Span::raw(" "),
					Span::styled(flattened_context, Style::default().fg(Color::Gray)),
				]))
			})
			.collect();

		// Create a List from all list items and highlight the currently selected one
		let list = List::new(items)
			.block(Block::default()
				.borders(Borders::ALL)
				.title(format!(" [ {} ] [ {} / {} ] ",
					self.input_filename,
					self.list.state().selected().unwrap_or(0) + 1,
					self.list.items().len()
				))
			)
			.highlight_style(Style::default().bg(Color::White).fg(Color::Black));

		// We can now render the item list
		f.render_stateful_widget(list, f.size(), self.list.state_mut());
	}
}
