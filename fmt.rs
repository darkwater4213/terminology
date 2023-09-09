use std::{fmt, path};

struct LogMessage {
    nb: i32
}

impl LogMessage {
    fn new(nb: i32) -> Foo {
        Foo {
            nb,
        }
    }
}

impl fmt::Display for LogMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // We need to remove "-" from the number output.
        let tmp = self.nb.abs().to_string();

        formatter.pad_integral(self.nb >= 0, "Foo ", &tmp)
    }
}

impl fmt::Display for path {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            is_symlink() => todo!()
        }
    }
}

assert_eq!(&format!("{}", Foo::new(2)), "2");
assert_eq!(&format!("{}", Foo::new(-1)), "-1");
assert_eq!(&format!("{}", Foo::new(0)), "0");
assert_eq!(&format!("{:#}", Foo::new(-1)), "-Foo 1");
assert_eq!(&format!("{:0>#8}", Foo::new(-1)), "00-Foo 1");
