#[allow(unused)]
pub(crate) static TAB: &str = "    ";
#[allow(unused)]
pub(crate) static ESC: &str = "\u{1b}";

#[allow(unused)]
#[allow(non_snake_case)]
pub(crate) mod RESET {
	pub(crate) static ALL: &str		= "\u{1b}[0m";
	pub(crate) static COLOR: &str	= "\u{1b}[49m";
	pub(crate) static BOLD: &str	= "\u{1b}[21m";
	pub(crate) static DIM: &str		= "\u{1b}[22m";
	pub(crate) static ITALIC: &str	= "\u{1b}[23m";
	pub(crate) static UL: &str		= "\u{1b}[24m";
}

#[allow(unused)]
#[allow(non_snake_case)]
pub(crate) mod COLOR {
	pub(crate) static BLACK: &str	= "\u{1b}[30m";
	pub(crate) static RED: &str		= "\u{1b}[31m";
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
	pub(crate) static RED: &str		= "\u{1b}[91m";
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
	pub(crate) static FILE: &str	= "\u{1b}[0;1;97m";
	pub(crate) static EXEC: &str	= "\u{1b}[0;1;92m";
	pub(crate) static SYMLINK: &str	= "\u{1b}[0;1;96m";

	pub(crate) static BOLD: &str	= "\u{1b}[1m";
	pub(crate) static DIM: &str		= "\u{1b}[2m";
	pub(crate) static ITALIC: &str	= "\u{1b}[3m";
	pub(crate) static UL: &str		= "\u{1b}[4m";
}

// pub(crate) mod types;
// pub mod log;

// enum Effect {
//     Bold,
//     Dim,
//     Italic,
//     Underline,
//     Blink,
// }

// enum Reset {
//     All,
//     Color,
//     Bold,
//     Dim,
//     Italic,
//     Underline,
//     Blink,
// }

// enum Hue {
//     Black,
//     Red,
//     Green,
//     Yellow,
//     Blue,
//     Purple,
//     Cyan,
//     White,
// }

// struct Color {
//     bright: bool,
//     hue: Hue,
// }

// struct Style {
//     color: Color,
//     style: Effect,
// }


