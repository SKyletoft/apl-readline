use anyhow::Result;
use smallvec::SmallVec;
use std::{
	io::{Stdout, Write as _},
	process::{ChildStdin, Stdio},
	time::Duration,
};
use termion::{
	event::{Event, Key},
	input::TermRead,
	raw::{IntoRawMode, RawTerminal},
};

pub mod apl_symbols;

const OFF: &[char] = &['⎕', 'O', 'F', 'F'];

fn main() {
	let mut dyalog = std::process::Command::new("dyalog")
		.stdin(Stdio::piped())
		.spawn()
		.unwrap();
	let mut inner_stdin = dyalog.stdin.take().unwrap();
	std::thread::sleep(Duration::from_millis(200));

	let mut history = Vec::new();
	while let Some(line) = read_line(&history).expect("Writing to dyalog failed") {
		if !line.is_empty() {
			history.push(leak(line.clone()));
		}

		let write = |mut inner_stdin: &mut ChildStdin, line: &[char]| {
			write!(&mut inner_stdin, "\r\n")?;
			for c in line.iter() {
				write!(&mut inner_stdin, "{c}")?;
			}
			writeln!(&mut inner_stdin)
		};
		match write(&mut inner_stdin, &line) {
			Ok(_) => {}
			Err(_) => {
				eprintln!("Dyalog died, restarting");
				history.remove(history.len() - 2);
				dyalog = std::process::Command::new("dyalog")
					.stdin(Stdio::piped())
					.spawn()
					.unwrap();
				inner_stdin = dyalog.stdin.take().unwrap();
				for line in history.iter().filter(|l| l.contains(&'←')) {
					write(&mut inner_stdin, &line).expect("Dyalog died, retry failed");
				}
			}
		}

		inner_stdin.flush().unwrap();
		if line.as_slice() == OFF {
			break;
		}
	}

	std::thread::sleep(Duration::from_millis(500));
}

fn read_line(history: &[&'static [char]]) -> Result<Option<SmallVec<[char; 20]>>> {
	std::thread::sleep(Duration::from_millis(200));

	let mut history_index = history.len();
	let mut idx = 0usize;

	let mut out = SmallVec::new();
	let mut raw_mode_handle = std::io::stdout().into_raw_mode().unwrap();
	rerender_line(&mut raw_mode_handle, &out, idx).expect("Writing to dyalog failed");

	macro_rules! tab_expand {
		() => {{
			// if-let chains when?
			if idx >= 2 {
				if let Some(last_two) = &out.get(idx - 2..idx) {
					if let Some((_, _, apl_char)) = apl_symbols::APL_SYMBOLS
						.iter()
						.find(|(first, second, _)| *first == last_two[0] && *second == last_two[1])
					{
						out.remove(idx - 2);
						out.remove(idx - 2);
						out.insert(idx - 2, *apl_char);
						idx -= 1;
					}
				}
			}
		}};
	}

	for c in std::io::stdin().events() {
		let c = c?;
		match c {
			Event::Key(Key::Left) => {
				idx = idx.saturating_sub(1);
			}
			Event::Key(Key::Right) => {
				idx = (idx + 1).min(out.len());
			}
			Event::Key(Key::Ctrl('a' | 'A')) => {
				idx = 0;
			}
			Event::Key(Key::Ctrl('e' | 'E')) => {
				idx = out.len();
			}
			Event::Key(Key::Up | Key::Ctrl('p' | 'P')) => {
				history_index = history_index.saturating_sub(1);
				if let Some(line) = history.get(history_index) {
					out.clear();
					for &c in line.iter() {
						out.push(c);
					}
					idx = idx.min(out.len());
				}
			}
			Event::Key(Key::Ctrl('l')) => {
				write!(
					&mut raw_mode_handle,
					"{}{}",
					termion::clear::All,
					termion::cursor::Goto(1, 1)
				)?;
			}
			Event::Key(Key::Down | Key::Ctrl('n' | 'N')) => {
				history_index = (history_index + 1).max(history.len());
				if let Some(line) = history.get(history_index) {
					out.clear();
					for &c in line.iter() {
						out.push(c);
					}
					idx = idx.min(out.len());
				}
			}
			Event::Key(Key::Char('\n')) => {
				write!(&mut raw_mode_handle, "{}", termion::clear::CurrentLine)?;
				raw_mode_handle.flush()?;
				break;
			}
			Event::Key(Key::Char('\t')) => {
				tab_expand!();
			}
			Event::Key(Key::Ctrl('c' | 'C')) => {
				if out.is_empty() {
					raw_mode_handle.suspend_raw_mode().unwrap();
					std::process::exit(-1);
				} else {
					out.clear();
					idx = 0;
				}
			}
			Event::Key(Key::Ctrl('d' | 'D')) => {
				out.clear();
				for &c in OFF.iter() {
					out.push(c);
				}
				break;
			}
			Event::Key(Key::Delete) => {
				if idx == out.len() {
					continue;
				}
				out.remove(idx);
			}
			Event::Key(Key::Backspace) => {
				if idx == 0 {
					continue;
				}
				out.remove(idx - 1);
				idx = idx.saturating_sub(1);
			}
			Event::Key(Key::Char(c)) => {
				if idx == out.len() {
					out.push(c);
				} else {
					out.insert(idx, c);
				}
				idx += 1;

				tab_expand!();
			}
			Event::Key(_) | Event::Mouse(_) | Event::Unsupported(_) => {}
		}
		rerender_line(&mut raw_mode_handle, &out, idx).expect("Writing to dyalog failed");
	}

	std::mem::drop(raw_mode_handle);

	Ok(Some(out))
}

fn leak(s: SmallVec<[char; 20]>) -> &'static [char] {
	s.as_slice().to_owned().leak()
}

fn rerender_line(raw: &mut RawTerminal<Stdout>, s: &[char], idx: usize) -> Result<()> {
	write!(raw, "{}\r> ", termion::clear::CurrentLine,)?;
	for &c in s.iter() {
		write!(raw, "{c}")?;
	}
	write!(
		raw,
		"{}{}",
		termion::cursor::Left(1000),
		termion::cursor::Right((idx + 2) as _)
	)?;
	raw.flush()?;
	Ok(())
}
