mod app;
mod stateful_list;

use async_std::{
	channel,
	io::{BufReader, Error, /*Read, stdin*/},
	fs::File,
	prelude::*,
	task,
};
use app::App;
use clap::{Arg, Command};
use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::{Map, Value};
use std::time::Duration;
use tui::{
	backend::CrosstermBackend,
	Terminal,
};

fn main() -> Result<(), Error> {
	let cmd = command_line();
	let matches = cmd.get_matches();

	let path: String = matches.get_one::<String>("INPUT").unwrap().to_string();
	let (sx, rx) = channel::unbounded();

	let mut app = App::new(path.to_string(), &rx);

	// Read the input in a loop
	task::spawn(async move {
		// Read from input file or stdin
		// let bf: BufReader<dyn Read + 'static> = if path == "-" {
		// 	BufReader::new(stdin())
		// } else {
		// 	BufReader::new(File::open(&path).await?)
		// };

		let bf = File::open(&path).await?;

		let mut lines = BufReader::new(bf).lines();
		while let Some(line) = lines.next().await {
			let log = serde_json::from_str::<Map<String, Value>>(&line?)?;
			match sx.send(log).await {
				Ok(_) => (),
				Err(_) => break,
			}
		}

		Ok::<(), Error>(())
	});

	// setup terminal
	enable_raw_mode()?;
	let mut stdout = std::io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	// create app and run it
	let tick_rate = Duration::from_millis(250);
	let res = app.run(&mut terminal, tick_rate);

	// restore terminal
	disable_raw_mode()?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
	terminal.show_cursor()?;

	if let Err(err) = res {
		println!("{:?}", err)
	}

	Ok(())
}

fn command_line() -> Command {
	Command::new("json-log-viewer")
		.about("JSON log file viewer")
		.arg(Arg::new("INPUT")
			.help("the input file to use")
			.required(false)
			.default_value("-")
		)
}
