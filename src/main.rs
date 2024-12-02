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
	std::thread::sleep(Duration::from_millis(200));

	let mut history = Vec::new();
	while let Some(line) = read_line(&history) {
		if !line.is_empty() {
			history.push(line.clone());
		}

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
	let mut idx = 0usize;

	let mut out = SmallString::new();
	let mut raw_mode_handle = std::io::stdout().into_raw_mode().unwrap();
	rerender_line(&mut raw_mode_handle, &out, idx);

	for c in std::io::stdin().events() {
		let actual_length = out.chars().count();
		match c {
			Ok(Event::Key(Key::Left)) => {
				idx = idx.saturating_sub(1);
			}
			Ok(Event::Key(Key::Right)) => {
				idx = (idx + 1).min(actual_length);
			}
			Ok(Event::Key(Key::Ctrl('a' | 'A'))) => {
				idx = 0;
			}
			Ok(Event::Key(Key::Ctrl('e' | 'E'))) => {
				idx = actual_length;
			}
			Ok(Event::Key(Key::Up | Key::Ctrl('p' | 'P'))) => {
				history_index = history_index.saturating_sub(1);
				if let Some(line) = history.get(history_index) {
					out = line.clone();
					idx = idx.min(actual_length);
				}
			}
			Ok(Event::Key(Key::Down | Key::Ctrl('n' | 'N'))) => {
				history_index = (history_index + 1).max(history.len());
				if let Some(line) = history.get(history_index) {
					out = line.clone();
					idx = idx.min(actual_length);
				}
			}
			Ok(Event::Key(Key::Char('\n'))) => {
				let _ = raw_mode_handle
					.write(termion::clear::CurrentLine.as_ref())
					.unwrap();
				let _ = raw_mode_handle.write(b"\r").unwrap();
				raw_mode_handle.flush().unwrap();
				break;
			}
			Ok(Event::Key(Key::Char('\t'))) => {
				if out.len() < 2 {
					continue;
				}
				let last_two = &out.get(out.len() - 2..out.len());
				if let Some((_, apl_char)) = apl_symbols::APL_SYMBOLS
					.iter()
					.find(|(key, _)| Some(*key) == *last_two)
				{
					out.pop();
					out.pop();
					out.push(*apl_char);
					idx -= 1;
				}
			}
			Ok(Event::Key(Key::Ctrl('c' | 'C'))) => {
				if out.is_empty() {
					raw_mode_handle.suspend_raw_mode().unwrap();
					std::process::exit(-1);
				} else {
					out.clear();
					idx = 0;
				}
			}
			Ok(Event::Key(Key::Ctrl('d' | 'D'))) => {
				out.clear();
				out.push_str(OFF);
				break;
			}
			Ok(Event::Key(Key::Backspace)) => {
				if idx == actual_length {
					out.pop();
				} else if idx != 0 {
					let byte_idx = out.char_indices().nth(idx).map(|(i, _)| i).unwrap_or(0);
					out.remove(byte_idx);
				}
				idx = idx.saturating_sub(1);
			}
			Ok(Event::Key(Key::Char(c))) => {
				if idx == out.len() {
					out.push(c);
				} else {
					let byte_idx = out
						.char_indices()
						.nth(idx)
						.map(|(i, _)| i)
						.unwrap_or(out.len());
					out.insert(byte_idx, c);
				}
				idx += 1;
			}
			_ => {}
		}
		rerender_line(&mut raw_mode_handle, &out, idx);
	}

	std::mem::drop(raw_mode_handle);

	Some(out)
}

fn rerender_line(raw: &mut RawTerminal<Stdout>, s: &str, idx: usize) {
	write!(
		raw,
		"{}\r> {}{}{}",
		termion::clear::CurrentLine,
		s,
		termion::cursor::Left(1000),
		termion::cursor::Right((idx + 2) as _)
	)
	.unwrap();
	raw.flush().unwrap();
}
