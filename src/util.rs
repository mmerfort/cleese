// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Utilities
//!
//! An assortment of useful operations used throughout the project. These are
//! basically here because there is no other good place for them, and because
//! putting them in `src/irc` would be inconvenient for importing in
//! `src/plugins`.

#![stable]


/// Split a string on whitespace, excluding empty strings.
///
/// This makes sure that lifetimes are preserved for the original string slice.
///
/// ## Example
///
/// ```
/// let s = space_slit("This is a string");
/// for item in s.iter() {
///   println!("{}", item);
/// }
/// => This
/// => is
/// => a
/// => string
/// ```
pub fn space_split<'a>(s: &'a str) -> Vec<&'a str> {
    s.split(|c: char| -> bool {
        c == ' '
    }).filter(|s: &&str| -> bool {
        *s != ""
    }).collect()
}


/// Split a string on newlines, don't include empty lines.
///
/// This makes sure that lifetimes are preserved for the original string slice.
///
/// ## Example
///
/// ```
/// let s = newline_split("This\nis\na\nstring");
/// for item in s.iter() {
///   println!("{}", item);
/// }
/// => This
/// => is
/// => a
/// => string
/// ```
pub fn newline_split<'a>(s: &'a str) -> Vec<&'a str> {
    s.split(|c: char| -> bool {
        c == '\n'
    }).map(|s: &'a str| -> &'a str {
        s.trim()
    }).filter(|s: &&str| -> bool {
        *s != ""
    }).collect()
}


/// Join the strings in xs together, placing between in the middle of each
/// individual joined string pair.
///
/// This is the classic join operation. The only thing to be aware of is that
/// the final string is an owned string, not a slice.
///
/// ## Example
///
/// ```
/// let s = join(&vec!["This", "is", "a", "string"], " ");
/// println!("{}", item);
/// => This is a string
/// ```
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
///
/// This is the classic join operation, and both the original vector and the
/// final string are composed of owned strings.
///
/// ## Example
///
/// ```
/// let first = "Hello ".to_string();
/// let second = "World!".to_string();
/// let s = join(&vec![first, second], " ");
/// println!("{}", item);
/// => This is a string
/// ```
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

