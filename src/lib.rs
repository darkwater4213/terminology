#[allow(unused)]
pub(crate) static TAB: &str = "    ";
#[allow(unused)]
pub(crate) static ESC: &str = "\u{1b}";

#[allow(unused)]
#[allow(non_snake_case)]
pub(crate) mod RESET {
	pub(crate) static ALL: &str	= "\u{1b}[0m";
	pub(crate) static COLOR: &str	= "\u{1b}[49m";
	pub(crate) static BOLD: &str	= "\u{1b}[21m";
	pub(crate) static DIM: &str	= "\u{1b}[22m";
	pub(crate) static ITALIC: &str	= "\u{1b}[23m";
	pub(crate) static UL: &str		= "\u{1b}[24m";
}

#[allow(unused)]
#[allow(non_snake_case)]
pub(crate) mod COLOR {
	pub(crate) static BLACK: &str	= "\u{1b}[30m";
	pub(crate) static RED: &str	= "\u{1b}[31m";
	pub(crate) static GREEN: &str	= "\u{1b}[32m";
	pub(crate) static YELLOW: &str	= "\u{1b}[33m";
	pub(crate) static BLUE: &str	= "\u{1b}[34m";
	pub(crate) static PURPLE: &str	= "\u{1b}[35m";
	pub(crate) static CYAN: &str	= "\u{1b}[36m";
	pub(crate) static WHITE: &str	= "\u{1b}[37m";
}

#[allow(unused)]
#[allow(non_snake_case)]
pub(crate) mod BRIGHT {
	pub(crate) static BLACK: &str	= "\u{1b}[90m";
	pub(crate) static RED: &str	= "\u{1b}[91m";
	pub(crate) static GREEN: &str	= "\u{1b}[92m";
	pub(crate) static YELLOW: &str	= "\u{1b}[93m";
	pub(crate) static BLUE: &str	= "\u{1b}[94m";
	pub(crate) static PURPLE: &str	= "\u{1b}[95m";
	pub(crate) static CYAN: &str	= "\u{1b}[96m";
	pub(crate) static WHITE: &str	= "\u{1b}[97m";
}

#[allow(unused)]
#[allow(non_snake_case)]
pub(crate) mod STYLE {
	pub(crate) static DIR: &str		= "\u{1b}[0;1;94m";
	pub(crate) static FILE: &str		= "\u{1b}[0;1;97m";
	pub(crate) static EXEC: &str		= "\u{1b}[0;1;92m";
	pub(crate) static SYMLINK: &str	= "\u{1b}[0;1;96m";

	pub(crate) static BOLD: &str	= "\u{1b}[1m";
	pub(crate) static DIM: &str	= "\u{1b}[2m";
	pub(crate) static ITALIC: &str	= "\u{1b}[3m";
	pub(crate) static UL: &str		= "\u{1b}[4m";
}

mod types {

	// HACK: finish the impl below
	// impl std::fmt::Display for FileName<&str> {
	// 	fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> { todo!() }
	// }
	// #[derive(Display)]
	pub enum FileName<T> {
		Regular(T),
		Executable(T),
		Directory(T),
		Symlink(T),
		Pathname(T),
	}

	// TODO: create an expandable tuple struct or similar (vector?)
	// TODO: for composing FileName<String> to a PathName

	impl FileName<String> {
		pub fn to_string(self) -> String {
			use FileName::*;
			use super::STYLE::*;
			let mut result: String = String::new();
			result = match self {
				Regular(inner) => result + FILE + &inner,
				Executable(inner) => result + EXEC + &inner,
				Directory(inner) => result + DIR + &inner,
				Symlink(inner) => result + SYMLINK + &inner,
				Pathname(inner) => result + "" + &inner,
			};
			result + super::RESET::ALL
		}
	}

	trait ToFileName {
		fn to_filename(&self) -> self::FileName<String>;
		fn to_exec_name(&self) -> self::FileName<String>;
		fn to_directory(&self) -> self::FileName<String>;
		fn to_symlink(&self, target: FileName<String>) -> self::FileName<String>;
	}

	trait ToPathName {
		fn to_pathname(&self) -> self::FileName<String>;
	}

	impl ToFileName for String {
		fn to_filename(&self) -> self::FileName<String> {
			let inner = super::STYLE::FILE.to_string() + &self + super::RESET::ALL;
			let result = FileName::Regular(inner);
			result
		}

		fn to_exec_name(&self) -> self::FileName<String> {
			let inner = super::STYLE::EXEC.to_string() + &self + super::RESET::ALL;
			let result = FileName::Executable(inner);
			result
		}

		fn to_directory(&self) -> self::FileName<String> {
			let inner = super::STYLE::DIR.to_string() + &self +
				match &self.chars().last().expect("Converting String to FileName::Directory failed!") {
					'/'	=> "",
					_	=> "/",
				}
				+ super::RESET::ALL;
			let result = FileName::Directory(inner);
			result
		}

		fn to_symlink(&self, target: FileName<String>) -> self::FileName<String> {
			let inner = super::STYLE::SYMLINK.to_string() +
				// if ! target exists { super::COLOR::RED } else { "" } +
				&*self + super::RESET::ALL +
				" -> " + &target.to_string() + super::RESET::ALL;
			let result = FileName::Symlink(inner);
			result
		}
	}

	// TODO: fix the impl below
	// TODO: check that only the last FileName is NOT a FileName::Directory
	// impl ToPathName for FileName<String> {
	// 	fn to_pathname(&self) -> FileName<String> {
	// 		let mut inner: String = (&self).to_string().to_string();
	// 		for i in [ 1 ..= inner.len() - 1 ] {
	// 			inner.push(self.i);
	// 		}
	// 		let result = FileName::Pathname(inner);
	// 		result
	// 	}
	// }

	pub struct LogEntry {
		pub text: String,
		pub direct: FileName<String>,
		pub infix: String,
		pub indirect: FileName<String>,
		pub postfix: String,
	}

	pub enum LogType {
		Header(LogEntry),
		Info(LogEntry),
		Hint(LogEntry),
		Message(LogEntry),
		Action(LogEntry),
		Warning(LogEntry),
		Error(LogEntry),
		Prompt(LogEntry)
	}

	impl LogEntry {
		// TODO: impl some constructors for LogEntry
		// fn new() -> LogEntry {
		// 	let mut result = LogEntry {
		// 		text: "",
		// 		direct: FileName::Pathname(""),
		// 		infix: "",
		// 		indirect: FileName::Pathname(""),
		// 		postfix: "",
		// 	};
		// 	result
		// }
	}
}


// types: directory filename executable warning error prompt 

pub mod log {
	// use super::TAB;
		use super::BRIGHT::{BLUE, GREEN, YELLOW, CYAN};
		use super::COLOR::YELLOW as ORANGE;
		use super::COLOR::GREEN as DARKGREEN;
		use super::COLOR::{RED, PURPLE};
		use super::STYLE::BOLD;
		use super::RESET::ALL as RESET;
		use super::types::LogType;

	trait Loggable {
		fn log(self);
	}

	impl Loggable for LogType {
		fn log(self) {
			use LogType::*;
			// use super::types::LogEntry;
			// use super::RESET::COLOR;
			// FIXME: figure out a PAINLESS way to capture inner from the match self{} expression.
			match self {
				Header(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					BLUE, BOLD, "[#]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				Info(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					CYAN, BOLD, "[*]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				Hint(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					DARKGREEN, BOLD, "[+]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				Message(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					GREEN, BOLD, "[>]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				Action(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					YELLOW, BOLD, "[@]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				Warning(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					ORANGE, BOLD, "[!]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				Error(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					RED, BOLD, "[!]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				Prompt(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
					PURPLE, BOLD, "[?]", RESET, // :
					inner.text,
					inner.direct.to_string(),
					inner.infix,
					inner.indirect.to_string(),
					inner.postfix
				),
				// This is a catchall for unimplemented message types
				// Enable if necessary.
				// LogType::_(inner) => println!( "{}{}{}{}: {}{}{}{}{}",
				// 	COLOR, BOLD, "[_]", RESET, // :
				// 	inner.text,
				// 	inner.direct.to_string(),
				// 	inner.infix,
				// 	inner.indirect.to_string(),
				// 	inner.postfix
				// ),
			};
		}
	}
}

// #[test]
// fn check_dirname() {
//	use types::FileName;
// 	println!("Tests:\n{} {}\n{} {}\n",
// 		"dir", "dir".to_directory(),
// 		"dir/", "dir/".to_directory(),
// 	)
// }