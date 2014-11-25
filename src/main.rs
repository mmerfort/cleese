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
use std::io::{mod};
use regex::Regex;
use getopts::{optopt, optflag, getopts, usage};
use irc::*;

mod irc;
mod util;
mod plugins;

static CMD_PREFIX: char = '.';

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
    let usage = usage("Starts cleese, an IRC bot written in rust.", &opts);

    let config = match matches.opt_str("c") {
        Some(c) => c,
        None => "config.json".to_string()
    };

    if matches.opt_present("help") {
        help(progname.as_slice(), usage.as_slice())
    } else if matches.opt_present("version") {
        version()
    } else {
        run(config)
    };
}

fn run(config: String) {
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

    // Connect to the IRC channel
    let mut irc = Irc::connect(conf);

    // Register some heavier plugins
    plugins::register(&mut irc);

    irc.run();
}

fn help(progname: &str, usage: &str) {
    println!("Usage: {} [OPTION]", progname);
    io::stdio::println(usage);
}

fn version() {
    println!("cleese 0.0.1");
}

