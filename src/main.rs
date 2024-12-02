use std::{
	io::{Stdout, Write as _},
	process::Stdio,
	time::Duration,
};
use termion::{
	event::{Event, Key},
	input::TermRead,
	raw::{IntoRawMode, RawTerminal},
};

type SmallString = smallstr::SmallString<[u8; 16]>;

pub mod apl_symbols;

const OFF: &str = "⎕OFF";

fn main() {
	let mut dyalog = std::process::Command::new("dyalog")
		.stdin(Stdio::piped())
		.spawn()
		.unwrap();
	let mut inner_stdin = dyalog.stdin.take().unwrap();

	let mut history = Vec::new();
	while let Some(line) = read_line(&history) {
		history.push(line.clone());

		match writeln!(&mut inner_stdin, "\r\n{line}") {
			Ok(_) => {}
			Err(_) => {
				eprintln!("Dyalog died, restarting");
				history.remove(history.len() - 2);
				dyalog = std::process::Command::new("dyalog")
					.stdin(Stdio::piped())
					.spawn()
					.unwrap();
				inner_stdin = dyalog.stdin.take().unwrap();
				for line in history.iter().filter(|l| l.contains('←')) {
					writeln!(&mut inner_stdin, "\r\n{line}").expect("Dyalog died, retry failed");
				}
			}
		}

		inner_stdin.flush().unwrap();
		if line == OFF {
			break;
		}
	}

	std::thread::sleep(Duration::from_millis(500));
}

fn read_line(history: &[SmallString]) -> Option<SmallString> {
	std::thread::sleep(Duration::from_millis(200));

	let mut history_index = history.len();

	let mut out = SmallString::new();
	let mut raw_mode_handle = std::io::stdout().into_raw_mode().unwrap();
	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout();

	rerender_line(&mut raw_mode_handle, &out);
	for c in stdin.events() {
		let _ = stdout.flush();
		match c {
			Ok(Event::Key(Key::Up | Key::Ctrl('p' | 'P'))) => {
				history_index = history_index.saturating_sub(1);
				if let Some(line) = history.get(history_index) {
					out = line.clone();
					rerender_line(&mut raw_mode_handle, &out);
				}
			}
			Ok(Event::Key(Key::Down | Key::Ctrl('n' | 'N'))) => {
				history_index = (history_index + 1).max(history.len());
				if let Some(line) = history.get(history_index) {
					out = line.clone();
					rerender_line(&mut raw_mode_handle, &out);
				}
			}
			Ok(Event::Key(Key::Char('\n'))) => {
				let _ = raw_mode_handle.write(termion::clear::CurrentLine.as_ref()).unwrap();
				let _ = raw_mode_handle.write(b"\r");
				raw_mode_handle.flush().unwrap();
				break;
			}
			Ok(Event::Key(Key::Char('\t'))) => {
				if out.len() < 2 {
					continue;
				}
				let last_two = &out[out.len() - 2..out.len()];
				if let Some((_, apl_char)) = apl_symbols::APL_SYMBOLS
					.iter()
					.find(|(key, _)| *key == last_two)
				{
					out.pop();
					out.pop();
					out.push(*apl_char);
					rerender_line(&mut raw_mode_handle, &out);
				}
			}
			Ok(Event::Key(Key::Ctrl('c' | 'C'))) => {
				std::mem::drop(std::io::stdout().into_raw_mode());
				std::process::exit(-1);
			}
			Ok(Event::Key(Key::Ctrl('d' | 'D'))) => {
				out.clear();
				out.push_str(OFF);
				break;
			}
			Ok(Event::Key(Key::Backspace)) => {
				out.pop();
				rerender_line(&mut raw_mode_handle, &out);
			}
			Ok(Event::Key(Key::Char(c))) => {
				out.push(c);
				rerender_line(&mut raw_mode_handle, &out);
			}
			_ => {}
		}
	}

	std::mem::drop(raw_mode_handle);

	Some(out)
}

fn rerender_line(raw: &mut RawTerminal<Stdout>, s: &str) {
	let _ = raw.write(termion::clear::CurrentLine.as_ref()).unwrap();
	let _ = raw.write(b"\r> ");
	let _ = raw.write(s.as_bytes()).unwrap();
	raw.flush().unwrap();
}
