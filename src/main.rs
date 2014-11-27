// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Cleese: The Friendly IRC Bot
//!
//! Cleese is a friendly IRC bot designed for easy creation and usage of
//! plugins. He exists for use in the CSUSB CSE Club IRC Channel,
//! and was written by Andrew Brinker based on work done for Rustbot
//! by [Jonas Hietala](https://github.com/treeman/rustbot).
//!
//! ## Structure
//!
//! Cleese is structured as a fairly simple core controller that passes commands
//! to a series of plugins which handle them. The cleese core (defined in
//! `src/irc`) handles parsing of the configuration file and connection to the
//! IRC server and channel. It then listens for commands prefixed with the
//! configured prefix and when encountered passes those commands on to the
//! plugin subsystem.
//!
//! ## Plugin System
//!
//! Plugins are defined `src/plugins` and registered in `src/plugins/mod.rs`.
//! When a command is encountered the registered plugins are iterated through,
//! and each one checks if the commands matches their accepted commands. If it
//! does, the command is processed by that plugin and does not pass on to the
//! next. If no command processes the plugin then Cleese continues silently.
//!
//! ## Multiple Connections
//!
//! As it stands, Cleese can only connect to a single server at a time.


#![crate_type = "bin"]
#![crate_name = "cleese"]
#![comment = "Your friendly IRC bot"]
#![license = "MIT"]
#![doc(html_logo_url = "/Users/andrew/Projects/cleese/assets/logo.png",
       html_favicon_url = "/Users/andrew/Projects/cleese/assets/favicon.ico",
       html_root_url = "http://github.com/andrewbrinker/cleese")]

#![unstable]

#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

#![feature(globs, macro_rules, phase)]
#[phase(plugin)]

extern crate regex_macros;
extern crate regex;
extern crate serialize;
extern crate getopts;
extern crate core;
extern crate time;

use std::os;
use std::io::{mod, BufferedReader, File};
use regex::Regex;
use getopts::{optopt, optflag, getopts, usage, OptGroup};
use irc::*;

mod irc;
mod util;
mod plugins;


// Default file names to be used later. Defined at the top for simplicity.
static DEFAULT_CONF_FILE: &'static str = "config.json";
static CARGO_FILE: &'static str = "Cargo.toml";


/// Parse and respond to the CLI args
///
/// The entry point for the program. Parses the command line arguments, and then
/// either prints the help text, prints the version, or starts the IRC bot.
///
/// ## Example
///
/// ```
/// main()
/// ```
fn main() {
    let args = os::args();

    let opts = [
        optopt("c", "config", "Specify config file", "CFILE"),
        optflag("v", "version", "Output version information and exit"),
        optflag("h", "help", "Display this help and exit")
    ];
    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => m,
        Err(e) => panic!("{}", e)
    };

    let progname = args[0].clone();

    let config_file = match matches.opt_str("c") {
        Some(c) => c,
        None => DEFAULT_CONF_FILE.to_string()
    };
    let jconf = JsonConfig::new(config_file);

    let config = IrcConfig {
        host:     jconf.host.as_slice(),
        port:     jconf.port,
        nick:     jconf.nick.as_slice(),
        descr:    jconf.descr.as_slice(),
        channels: jconf.channels.iter().map(|x| x.as_slice()).collect(),

        // Input blacklist by code.
        in_blacklist: jconf.in_blacklist.iter().map(|x| x.as_slice()).collect(),

        // Output is blacklisted with regexes, as they lack structure.
        out_blacklist: jconf.out_blacklist.iter().map(
            |x| {
                match Regex::new(x.as_slice()) {
                    Ok(re) => re,
                    Err(err) => panic!("{}", err),
                }
            }).collect(),
        cmd_prefix: jconf.cmd_prefix.as_slice(),
    };

    if matches.opt_present("help") {
        help(progname.as_slice(), &opts, config.descr)
    } else if matches.opt_present("version") {
        version()
    } else {
        run(config)
    };
}


/// Run the IRC bot
///
/// This works by loading in the configuration from the config file,
/// connecting to the server, and initializing all registered plugins.
///
/// ## Example
///
/// ```
/// let config = IrcConfig {
///     // ...
/// }
///
/// run(config);
/// ```
fn run(config: IrcConfig) {
    let mut irc = Irc::connect(config);
    plugins::register(&mut irc);
    irc.run();
}


/// Print the program help text
///
/// This takes in the program name, command-line options, and program
/// description, and uses them to print a nice complete help text. The
/// description is loaded from config.json (or the alternative config file if
/// you've defined one), so make sure your config file actually defines a
/// description.
///
/// ## Example
///
/// ```
/// help("cleese", &opts, "Your friendly neighborhood IRC bot");
/// ```
fn help(progname: &str, opts: &[OptGroup], descr: &str) {
    let u = usage(format!("{}", descr).as_slice(), opts);
    println!("Usage: {} [OPTION]", progname);
    io::stdio::println(u.as_slice());
}


/// Print the current version.
///
/// This works by grabbing the version from the Cargo.toml file. If it can't
/// find the file it will error out. Make sure you actually have a Cargo.toml
/// file (which you should), and that the Cargo.toml file always have an
/// up-to-date version.
///
/// ## Example
///
/// ```
/// version()
/// ```
fn version() {
    let file = File::open(&Path::new(CARGO_FILE));
    let mut reader = BufferedReader::new(file.unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("version") {
            let version = regex!("\".*?\"").find(line.as_slice()).unwrap();
            println!("{}", line.slice(version.0 + 1u, version.1 - 1u));
        }
    }
}

