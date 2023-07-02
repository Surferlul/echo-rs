//! Basic rewrite of the echo GNU core utility, maintaining compatability

use std::{
    env::args,
    io::{self, Write},
    process::exit,
};

mod consts;

use crate::consts::{HELP_DIALOG, HEX_REGEX, OCTAL_REGEX, SIMPLE_SPECIAL_SEQUENCES, VERSION};

/// Settings to be read from cli
struct Settings {
    trailing_newline: bool,
    interpret_backslash_escapes: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            trailing_newline: true,
            interpret_backslash_escapes: false,
        }
    }
}

/// Replaces escaped octal representations with corresponding characters
fn replace_octal(string: String) -> String {
    let mut res = string;
    while let Some(captures) = OCTAL_REGEX.captures(
        #[allow(clippy::redundant_clone)]
        &res.clone(),
    ) {
        if let (Some(entire_match), Some(capture)) = (captures.get(0), captures.get(1)) {
            res = format!(
                "{}{}{}",
                &res[..entire_match.start()],
                &u8::from_str_radix(capture.as_str(), 8)
                    .map_or_else(|_| 255 as char, |v| v as char),
                &res[entire_match.end()..],
            );
        } else {
            println!("error matching octal regex. aborting");
            exit(1);
        }
    }
    res
}

/// Replaces escaped hex representations with corresponding characters
fn replace_hex(string: String) -> String {
    let mut res = string;
    while let Some(captures) = HEX_REGEX.captures(
        #[allow(clippy::redundant_clone)]
        &res.clone(),
    ) {
        if let (Some(entire_match), Some(capture)) = (captures.get(0), captures.get(1)) {
            res = format!(
                "{}{}{}",
                &res[..entire_match.start()],
                &u8::from_str_radix(capture.as_str(), 16)
                    .map_or_else(|_| 255 as char, |v| v as char),
                &res[entire_match.end()..],
            );
        } else {
            println!("error matching hex regex. aborting");
            exit(1);
        }
    }
    res
}

/// Format passed argument based on settings
fn format_arg(arg: String, settings: &mut Settings) -> String {
    if settings.interpret_backslash_escapes {
        let mut res = arg;
        let mut backslash_escaped: Vec<String> =
            res.split(r#"\\"#).map(|s| s.to_string()).collect();
        let mut found_c = false;
        for element in &mut backslash_escaped {
            if found_c {
                *element = String::new();
                continue;
            }

            for sequence in SIMPLE_SPECIAL_SEQUENCES {
                *element = element.replace(sequence.0, sequence.1)
            }

            if let Some(pos) = element.find(r#"\c"#) {
                found_c = true;
                settings.trailing_newline = false;
                *element = element[..pos].to_string();
                continue;
            }

            *element = replace_octal(element.clone());
            *element = replace_hex(element.clone());
        }
        res = backslash_escaped.join(r#"\"#);
        res
    } else {
        arg
    }
}

/// Write string to stdout as unicode characters
fn write_as_unicode(string: String) {
    let stdout = io::stdout();
    if let Err(e) = stdout.lock().write_all(
        string
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>()
            .as_slice(),
    ) {
        eprintln!("error writing to stdout: {e}");
        exit(1)
    }
}

fn main() {
    let mut settings = Settings::default();
    let mut read_flags = true;
    for i in 1..args().len() {
        if let Some(arg) = args().nth(i) {
            if read_flags {
                match arg.as_str() {
                    "-n" => {
                        settings.trailing_newline = false;
                    }
                    "-e" => {
                        settings.interpret_backslash_escapes = true;
                    }
                    "-E" => {
                        settings.interpret_backslash_escapes = false;
                    }
                    "--help" => {
                        println!("{}", HELP_DIALOG);
                        exit(0)
                    }
                    "--version" => {
                        match VERSION {
                            Some(v) => {
                                println!("gnu-echo-rs {}", v)
                            }
                            None => {
                                println!("gnu-echo-rs was not compiled with a version")
                            }
                        }
                        exit(0)
                    }
                    _ => {
                        write_as_unicode(format_arg(arg, &mut settings));
                        read_flags = false;
                    }
                }
            } else {
                write_as_unicode(format!(" {}", format_arg(arg, &mut settings)));
            }
        }
    }
    if settings.trailing_newline {
        println!()
    }
}
