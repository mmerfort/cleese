// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Utilities
//!
//! An assortment of useful operations used throughout the project.

#![stable]

use std::{str, io};

/// Split a string on whitespace, excluding empty strings.
pub fn space_split<'a>(s: &'a str) -> Vec<&'a str> {
    s.split(|c: char| -> bool {
        c == ' '
    }).filter(|s: &&str| -> bool {
        *s != ""
    }).collect()
}

/// Split a string on newlines, don't include empty lines.
pub fn newline_split<'a>(s: &'a str) -> Vec<&'a str> {
    s.split(|c: char| -> bool {
        c == '\n'
    }).map(|s: &'a str| -> &'a str {
        s.trim()
    }).filter(|s: &&str| -> bool {
        *s != ""
    }).collect()
}


/// Run an external command and fetch its output.
pub fn run_external_cmd(cmd: &str, args: &[&str]) -> String {
    let mut process = match io::process::Command::new(cmd).args(args).spawn() {
        Ok(p) => p,
        Err(e) => panic!("Runtime error: {}", e),
    };

    let output = process.stdout.as_mut().unwrap().read_to_end();
    match output {
        Ok(x) => {
            str::from_utf8(x.as_slice()).unwrap().to_string()
        },
        Err(e) => panic!("Read error: {}", e),
    }
}

/// Join the strings in xs together, placing between in the middle of each
/// individual joined string pair.
pub fn join(xs: &Vec<&str>, between: &str) -> String {
    let mut res = String::new();
    for x in xs.iter() {
        if !res.is_empty() {
            res.push_str(between);
        }
        res.push_str(*x);
    }
    return res;
}

/// Same as join, but working on owned strings instead of slices.
pub fn join_strings(xs: &Vec<String>, between: &str) -> String {
    let mut res = String::new();
    for x in xs.iter() {
        if !res.is_empty() {
            res.push_str(between);
        }
        res.push_str(x.as_slice());
    }
    return res;
}
