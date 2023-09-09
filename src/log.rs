// use super::TAB;
use std::fmt::Display;
use super::BRIGHT::{BLUE, GREEN, YELLOW, CYAN};
use super::COLOR::YELLOW as ORANGE;
use super::COLOR::GREEN as DARKGREEN;
use super::COLOR::{RED, PURPLE};
use super::STYLE::BOLD;
use super::RESET::ALL as RESET;
use super::types::LogType;

trait Loggable {
	fn fmt(self);
}

impl Display for LogType {
	fn fmt(self) {
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
