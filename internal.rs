// macro_rules! concat!($1: &str, $2: &str) {
//     () => ()
// }

pub static TAB: &str = "    ";
pub(super) static ESC: &str = "\u{1b}";

#[allow(non_snake_case)]
pub(super) mod RESET {
	pub(super) static ALL: &str		= "\u{1b}[0m";
	pub(super) static COLOR: &str	= "\u{1b}[49m";
	pub(super) static BOLD: &str	= "\u{1b}[21m";
	pub(super) static DIM: &str		= "\u{1b}[22m";
	pub(super) static ITALIC: &str	= "\u{1b}[23m";
	pub(super) static UL: &str		= "\u{1b}[24m";
}

#[allow(non_snake_case)]
pub(super) mod COLOR {
	pub(super) static BLACK: &str	= "\u{1b}[30m";
	pub(super) static RED: &str		= "\u{1b}[31m";
	pub(super) static GREEN: &str	= "\u{1b}[32m";
	pub(super) static YELLOW: &str	= "\u{1b}[33m";
	pub(super) static BLUE: &str	= "\u{1b}[34m";
	pub(super) static PURPLE: &str	= "\u{1b}[35m";
	pub(super) static CYAN: &str	= "\u{1b}[36m";
	pub(super) static WHITE: &str	= "\u{1b}[37m";
}

#[allow(non_snake_case)]
pub(super) mod BRIGHT {
	pub(super) static BLACK: &str	= "\u{1b}[90m";
	pub(super) static RED: &str		= "\u{1b}[91m";
	pub(super) static GREEN: &str	= "\u{1b}[92m";
	pub(super) static YELLOW: &str	= "\u{1b}[93m";
	pub(super) static BLUE: &str	= "\u{1b}[94m";
	pub(super) static PURPLE: &str	= "\u{1b}[95m";
	pub(super) static CYAN: &str	= "\u{1b}[96m";
	pub(super) static WHITE: &str	= "\u{1b}[97m";
}

#[allow(unused)]
#[allow(non_snake_case)]
pub(super) mod STYLE {
	pub(super) static DIR: &str		= "\u{1b}[0;1;94m";
	pub(super) static FILE: &str	= "\u{1b}[0;1;97m";
	pub(super) static EXEC: &str	= "\u{1b}[0;1;92m";
	pub(super) static SYMLINK: &str	= "\u{1b}[0;1;96m";

	pub(super) static BOLD: &str	= "\u{1b}[1m";
	pub(super) static DIM: &str		= "\u{1b}[2m";
	pub(super) static ITALIC: &str	= "\u{1b}[3m";
	pub(super) static UL: &str		= "\u{1b}[4m";
}
pub(super) use super::COLOR::{
    RED as MAROON,
    GREEN,
    BLUE as INDIGO,
    CYAN as TEAL,
    MAGENTA as PURPLE,
    YELLOW as ORANGE,
    BLACK,
    WHITE as LTGREY,
};
pub(super) use super::BRIGHT::{
    RED,
    GREEN as LTGREEN,
    CYAN,
    MAGENTA,
    YELLOW,
    BLACK as GREY,
    WHITE,
};
pub(crate) use super::RESET;
pub(crate) use super::STYLE::*;
