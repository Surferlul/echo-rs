use std::{
    env::args,
    io::{self, Write},
    process::exit,
};

mod consts;

use crate::consts::{HELP_DIALOG, HEX_REGEX, OCTAL_REGEX, SIMPLE_SPECIAL_SEQUENCES, VERSION};

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

fn replace_octal(string: String) -> String {
    let mut res = string;
    while let Some(captures) = OCTAL_REGEX.captures(
        #[allow(clippy::redundant_clone)]
        &res.clone(),
    ) {
        if let (Some(entire_match), Some(capture)) = (captures.get(0), captures.get(1)) {
            let mut contents = capture.as_str();
            if contents.is_empty() {
                contents = "0" // GNU echo interprets \0 as \00
            }
            res = format!(
                "{}{}",
                &res[..entire_match.start()],
                OCTAL_REGEX.replace(
                    &res[entire_match.start()..],
                    &u8::from_str_radix(contents, 8)
                        .map_or_else(|_| 255 as char, |v| v as char)
                        .to_string(),
                )
            );
        } else {
            println!("error matching octal regex. aborting");
            exit(1);
        }
    }
    res
}

fn replace_hex(string: String) -> String {
    let mut res = string;
    let mut search_at = 0;
    while let Some(captures) = HEX_REGEX.captures_at(
        #[allow(clippy::redundant_clone)]
        &res.clone(),
        search_at,
    ) {
        if let (Some(entire_match), Some(capture)) = (captures.get(0), captures.get(1)) {
            let contents = capture.as_str();
            if contents.is_empty() {
                search_at = entire_match.end();
                continue; // GNU echo does not interpret \x
            }
            res = format!(
                "{}{}",
                &res[..entire_match.start()],
                HEX_REGEX.replace(
                    &res[entire_match.start()..],
                    &u8::from_str_radix(contents, 16)
                        .map_or_else(|_| 255 as char, |v| v as char)
                        .to_string(),
                )
            );
        } else {
            println!("error matching hex regex. aborting");
            exit(1);
        }
    }
    res
}

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
                                println!("echo-rs {}", v)
                            }
                            None => {
                                println!("echo-rs was not compiled with a version")
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
