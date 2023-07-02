use std::process::exit;

use lazy_static::lazy_static;
use regex::Regex;

pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const HELP_DIALOG: &str = r#"
Usage: gnu-echo-rs [SHORT-OPTION]... [STRING]...
  or:  gnu-echo-rs LONG-OPTION
Echo the STRING(s) to standard output. Rust rewrite of GNU echo util.

  -n             do not output the trailing newline
  -e             enable interpretation of backslash escapes
  -E             disable interpretation of backslash escapes (default)
      --help     display this help and exit
      --version  output version information and exit

If -e is in effect, the following sequences are recognized:

  \\      backslash
  \a      alert (BEL)
  \b      backspace
  \c      produce no further output
  \e      escape
  \f      form feed
  \n      new line
  \r      carriage return
  \t      horizontal tab
  \v      vertical tab
  \0NNN   byte with octal value NNN (1 to 3 digits)
  \xHH    byte with hexadecimal value HH (1 to 2 digits)
"#;
pub const SIMPLE_SPECIAL_SEQUENCES: [(&str, &str); 8] = [
    (r#"\a"#, "\x07"),
    (r#"\b"#, "\x08"),
    (r#"\e"#, "\x1b"),
    (r#"\f"#, "\x0c"),
    (r#"\n"#, "\n"),
    (r#"\r"#, "\r"),
    (r#"\t"#, "\t"),
    (r#"\v"#, "\x0b"),
];
lazy_static! {
    pub static ref OCTAL_REGEX: Regex = Regex::new(r#"\\(?:([1-7][0-7]{0,2}|0[0-7]{0,3}))"#)
        .map_or_else(
            |e| {
                eprintln!(
                    "programming error: cannot compile regex pattern for octal regex match: {}",
                    e
                );
                exit(1);
            },
            |v| v
        );
    pub static ref HEX_REGEX: Regex = Regex::new(r#"\\x([0-9A-F]{0,2})"#).map_or_else(
        |e| {
            eprintln!(
                "programming error: cannot compile regex pattern for hex regex match: {}",
                e
            );
            exit(1);
        },
        |v| v
    );
}
