pub static TAB: &str = "    ";
pub static ESC: &str = "\u{1b}";

pub mod RESET {
	pub static ALL: &str	= "\u{1b}[0m";
	pub static COLOR: &str	= "\u{1b}[49m";
	pub static BOLD: &str	= "\u{1b}[21m";
	pub static DIM: &str	= "\u{1b}[22m";
	pub static ITALIC: &str	= "\u{1b}[23m";
	pub static UL: &str		= "\u{1b}[24m";
}

pub mod COLOR {
	pub static BLACK: &str	= "\u{1b}[30m";
	pub static RED: &str	= "\u{1b}[31m";
	pub static GREEN: &str	= "\u{1b}[32m";
	pub static YELLOW: &str	= "\u{1b}[33m";
	pub static BLUE: &str	= "\u{1b}[34m";
	pub static PURPLE: &str	= "\u{1b}[35m";
	pub static CYAN: &str	= "\u{1b}[36m";
	pub static WHITE: &str	= "\u{1b}[37m";
}

pub mod BRIGHT {
	pub static BLACK: &str	= "\u{1b}[90m";
	pub static RED: &str	= "\u{1b}[91m";
	pub static GREEN: &str	= "\u{1b}[92m";
	pub static YELLOW: &str	= "\u{1b}[93m";
	pub static BLUE: &str	= "\u{1b}[94m";
	pub static PURPLE: &str	= "\u{1b}[95m";
	pub static CYAN: &str	= "\u{1b}[96m";
	pub static WHITE: &str	= "\u{1b}[97m";
}

pub mod STYLE {
	pub static DIR: &str		= "\u{1b}[0;1;94m";
	pub static FILE: &str		= "\u{1b}[0;1;97m";
	pub static EXEC: &str		= "\u{1b}[0;1;92m";
	pub static SYMLINK: &str	= "\u{1b}[0;1;96m";
	// pub static NEWLINE: &str	= RESET + "\n";

	pub static BOLD: &str	= "\u{1b}[1m";
	pub static DIM: &str	= "\u{1b}[2m";
	pub static ITALIC: &str	= "\u{1b}[3m";
	pub static UL: &str		= "\u{1b}[4m";
}

mod types {

	pub enum FileName<T> {
		Regular(T),
		Executable(T),
		Directory(T),
		Symlink(T),
		Pathname(T),
	}

	// TODO: create an expandable tuple struct or similar (vector?)
	// TODO: for composing FileName<&str> to a PathName

	impl FileName<&str> {
		fn to_str(self) -> &str {
			use FileName::*;
			// let mut result: &str;
			if let Regular(result) = self {};
			if let Executable(result) = self {};
			if let Directory(result) = self {};
			if let Symlink(result) = self {};
			if let Pathname(result) = self {};
			result
		}
	}

	trait ToFileName {
		fn to_filename(&self) -> self::FileName<&str>;
		fn to_exec_name(&self) -> self::FileName<&str>;
		fn to_directory(&self) -> self::FileName<&str>;
		fn to_symlink(&self, target: FileName<&str>) -> self::FileName<&str>;
	}

	trait ToPathName {
		fn to_pathname(&self) -> self::FileName<&str>;
	}

	impl ToFileName for &str {
		fn to_filename(&self) -> self::FileName<&str> {
			let inner = super::STYLE::FILE.to_string() + &self + super::RESET::ALL;
			let result = FileName::Regular(&*inner);
			result
		}

		fn to_exec_name(&self) -> self::FileName<&str> {
			let inner = super::STYLE::EXEC.to_string() + &self + super::RESET::ALL;
			let result = FileName::Executable(&*inner);
			result
		}

		fn to_directory(&self) -> self::FileName<&str> {
			let inner = super::STYLE::DIR.to_string() + &self +
				match &self.chars().last().expect("Converting &str to FileName::Directory failed!") {
					'/'	=> "",
					_	=> "/",
				}
				+ super::RESET::ALL;
			let result = FileName::Directory(&*inner);
			result
		}

		fn to_symlink(&self, target: FileName<&str>) -> self::FileName<&str> {
			let inner = super::STYLE::SYMLINK.to_string() +
				// if ! target exists { super::COLOR::RED } else { "" } +
				*self + super::RESET::ALL +
				" -> " + target.to_str() + super::RESET::ALL;
			let result = FileName::Symlink(&*inner);
			result
		}
	}

	// TODO: fix the impl below
	// TODO: check that only the last FileName is NOT a FileName::Directory
	impl ToPathName for FileName<&str> {
		fn to_pathname(&self) -> self::FileName<&str> {
			let mut inner: String = (&self).to_str().to_string();
			for i in [ 1 ..= inner.len() - 1 ] {
				inner.push(&self[i]);
			}
			let result = FileName::Path(&*inner);
			result
		}
	}

	pub struct LogEntry<'a> {
		text: &'a str,
		direct: FileName<&'a str>,
		infix: &'a str,
		indirect: FileName<&'a str>,
		postfix: &'a str,
	}

	pub enum LogType<'a> {
		Header(LogEntry<'a>),
		Info(LogEntry<'a>),
		Hint(LogEntry<'a>),
		Message(LogEntry<'a>),
		Action(LogEntry<'a>),
		Warning(LogEntry<'a>),
		Error(LogEntry<'a>),
		Prompt(LogEntry<'a>)
	}

	impl LogEntry<'_> {
		fn new() -> LogEntry<'_> {
			let mut result = LogEntry {
				text: "",
				direct: FileName::Pathname(""),
				infix: "",
				indirect: FileName::Pathname(""),
				postfix: "",
			};
			result
		}
	}
}


// types: directory filename executable warning error prompt 

pub mod log {
	use super::TAB;
		use super::BRIGHT::{BLUE, GREEN, YELLOW, CYAN};
		use super::COLOR::YELLOW as ORANGE;
		use super::COLOR::GREEN as DARKGREEN;
		use super::COLOR::{RED, PURPLE};
		use super::STYLE::BOLD;
		use super::RESET::ALL as RESET;
		use super::RESET::COLOR;
		use super::types::LogType;

	trait Loggable {
		fn log(self);
	}

	impl Loggable for LogType<'_> {
		fn log(self) {
			use LogType::*;
			println!( "{}{}{}: {}{}{}{}{}",
				TAB, BOLD,
				match self {
					Header(LogEntry)	=> BLUE		.to_string() + "[#]",
					Info(LogEntry)		=> CYAN		.to_string() + "[*]",
					Hint(LogEntry)		=> DARKGREEN.to_string() + "[+]",
					Message(LogEntry)	=> GREEN	.to_string() + "[>]",
					Action(LogEntry)	=> YELLOW	.to_string() + "[@]",
					Warning(LogEntry)	=> ORANGE	.to_string() + "[!]",
					Error(LogEntry)		=> RED		.to_string() + "[!]",
					Prompt(LogEntry)	=> PURPLE	.to_string() + "[?]",
					_					=> COLOR	.to_string() + "[_]",
				} + RESET,
				self.text,
				self.direct,
				self.infix,
				self.indirect,
				self.postfix,
			)
		}
	}

	// OLD non-impl logging functions
	// use super::types::FileName
	// pub fn head<T>(header: FileName<&str>) {
	// 	println!( "{}{}[#]{}: {}",
	// 		super::TAB,
	// 		BLUE,
	// 		RESET,
	// 		<FileName<&str> as Into<T>>::into(header.to_str())
	// 	)
	// }
	// pub fn message<T>(message: FileName<&str>) {
	// 	println!( "{}{}[>]{}: {}",
	// 		super::TAB,
	// 		GREEN,
	// 		RESET,
	// 		<FileName<&str> as Into<T>>::into(message.to_str())
	// 	)
	// }
	// pub fn warn<T>(warning: FileName<&str>) {
	// 	eprintln!( "{}{}[!]{}: {}",
	// 		super::TAB,
	// 		ORANGE,
	// 		RESET,
	// 		<FileName<&str> as Into<T>>::into(warning.to_str())
	// 	)
	// }
	// pub fn enter_dir(directory: FileName<&str>) {
	// 	println!( "{}{}[@]{}: Entering {}",
	// 		super::TAB,
	// 		YELLOW,
	// 		RESET,
	// 		super::STYLE::DIR.to_string() + directory.to_str()
	// 	)
	// }
	// pub fn leave_dir(directory: FileName<&str>) {
	// 	println!( "{}{}[@]{}: Leaving {}",
	// 		super::TAB,
	// 		super::BRIGHT::YELLOW,
	// 		RESET,
	// 		super::STYLE::DIR.to_string() + directory.to_str()
	// 	)
	// }
	// pub fn err<T>(error: FileName<&str>) {
	// 	eprintln!( "{}{}[!]{}: {}",
	// 		super::TAB,
	// 		RED,
	// 		RESET,
	// 		<FileName<&str> as Into<T>>::into(error.to_str())
	// 	)
	// }
	// pub fn prompt<T>(prompt: FileName<&str>) {
	// 	println!( "{}{}[?]{}: {}",
	// 		super::TAB,
	// 		PURPLE,
	// 		RESET,
	// 		<FileName<&str> as Into<T>>::into(prompt.to_str())
	// 	);
	// }
}

#[test]
fn check_dirname() {
	println!("Tests:\n{} {}\n{} {}\n",
		"dir", "dir".to_directory(),
		"dir/", "dir/".to_directory(),
	)
}