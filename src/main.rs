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
#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]
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

static DEFAULT_CONF_FILE: &'static str = "config.json";
static CARGO_FILE: &'static str = "Cargo.toml";


/// The entry point for the program. Parses the command line arguments, and then
/// either prints the help text, prints the version, or starts the IRC bot.
fn main() {
    // Get the arguments from the command line.
    let args = os::args();

    // Setup the available options.
    let opts = [
        optopt("c", "config", "Specify config file", "CFILE"),
        optflag("v", "version", "Output version information and exit"),
        optflag("h", "help", "Display this help and exit")
    ];

    // Check options and panic if getting the options fails.
    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => m,
        Err(e) => panic!("{}", e)
    };

    // Set the program name.
    let progname = args[0].clone();

    // If a configuration file name was passed, use it instead of config.json.
    let config = match matches.opt_str("c") {
        Some(c) => c,
        None => DEFAULT_CONF_FILE.to_string()
    };

    // Read in the configuration file
    let jconf = JsonConfig::new(config);

    // Setup the configuration struct
    let conf = IrcConfig {
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
        cmd_prefix: jconf.cmd_prefix,
    };

    // Match against flags to either print help text, print version, or run the
    // IRC bot.
    if matches.opt_present("help") {
        help(progname.as_slice(), &opts, conf.descr)
    } else if matches.opt_present("version") {
        version()
    } else {
        run(conf)
    };
}

/// Run the IRC bot by loading in the configuration from the config file,
/// connecting to the server, and initializing all registered plugins.
fn run(config: IrcConfig) {
    let mut irc = Irc::connect(config);
    plugins::register(&mut irc);
    irc.run();
}

/// Print the help text using both the program name and the help info generated
/// by the usage() function earlier.
fn help<'a>(progname: &str, opts: &[OptGroup], descr: &'a str) {
    let u = usage(format!("{}", descr).as_slice(), opts);
    println!("Usage: {} [OPTION]", progname);
    io::stdio::println(u.as_slice());
}

/// Print the current version.
fn version() {
    let path = Path::new(CARGO_FILE);
    let file = File::open(&path);

    let mut cargo_file = BufferedReader::new(match file {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    });

    for line in cargo_file.lines() {
        let text = match line {
            Ok(x) => x,
            Err(e) => panic!("{}", e)
        };
        if text.starts_with("version") {
            let version = text.slice(11, text.len() - 2);
            println!("{}", version);
        }
    }
}

