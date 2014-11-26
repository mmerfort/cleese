// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Uptime
//!
//! A simple plugin for getting the bots uptime. When asked it calculates the
//! number of seconds since start-up, and converts that into a human-friendly
//! string to be output to the IRC chat.

extern crate time;

use irc::{IrcWriter, IrcCommand, BotInfo, Plugin};
use util;


/// Contains the starting time, initialized when the plugin is. This may not
/// actually perfectly coinside with the time since initial connection, but it's
/// close enough to be usable.
pub struct Uptime {
    start: time::Tm,
    description: &'static str,
    name: &'static str
}

impl Uptime {
    /// Construct a new Uptime struct with the time set to the current time.
    pub fn new() -> Uptime {
        Uptime {
            start: time::now(),
            description: "Find out how long the bot has been alive.",
            name: "Uptime"
        }
    }

    /// Get the uptime by subtracting the current time from the starting time
    /// and converting that into a human-friendly string.
    fn uptime(&self) -> String {
        let at = time::now();
        let dt = at.to_timespec().sec - self.start.to_timespec().sec;
        format!("I've been awake {}", format_duration(dt))
    }
}

impl Plugin for Uptime {
    /// Respond to received commands.
    ///
    /// Called by the plugin subsystem when a command is encountered. It only
    /// responds to the command "uptime". Otherwise it does nothing.
    fn cmd(&mut self, cmd: &IrcCommand, writer: &IrcWriter, _info: &BotInfo) {
        match cmd.name {
            "uptime" => {
                writer.msg(cmd.channel.as_slice(), self.uptime().as_slice());
            }
            _ => {}
        }
    }

    fn help(&self) -> &'static str { self.description }
    fn name(&self) -> &'static str { self.name }
}

/// Convert from an integer representation of seconds to a string describing the
/// associated time in natural language.
pub fn format_duration(mut sec: i64) -> String {
    let mut min: i64 = sec / 60;
    let mut hours: i64 = min / 60;
    let days: i64 = hours / 24;

    if sec > 0 {
        sec = sec - min * 60;
    }
    if hours > 0 {
        min = min - hours * 60;
    }
    if days > 0 {
        hours = hours - days * 24;
    }

    fn fmt(x: i64, s: &str) -> String {
        format!("{} {}{}", x, s, if x == 1 { "" } else { "s" })
    }
    let day_fmt = fmt(days, "day");
    let hour_fmt = fmt(hours, "hour");
    let min_fmt = fmt(min, "minute");
    let sec_fmt = fmt(sec, "second");

    let parts = {
        if days > 0 {
            vec![day_fmt, hour_fmt, min_fmt, sec_fmt]
        } else if hours > 0 {
            vec![hour_fmt, min_fmt, sec_fmt]
        } else if min > 0 {
            vec![min_fmt, sec_fmt]
        } else {
            vec![sec_fmt]
        }
    };
    util::join_strings(&parts, " ")
}
