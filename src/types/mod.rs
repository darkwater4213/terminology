// types: directory filename executable warning error prompt 
// use std::fs::*;

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

// #[test]
// fn check_dirname() {
//	use types::FileName;
// 	println!("Tests:\n{} {}\n{} {}\n",
// 		"dir", "dir".to_directory(),
// 		"dir/", "dir/".to_directory(),
// 	)
// }
