// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Command
//!
//! This defines the structure and processing of commands. Every command is
//! constructed in the following way:
//!
//!     prefix command args...
//!
//! The prefix may be any length, and the command is only one word. The prefix
//! is what identifies the text as a command. The rest (command and args) are
//! passed through to the plugin subsystem and processed there.

use irc::privmsg::*;
use util::*;

/// Command through IRC.
#[deriving(Show)]
pub struct IrcCommand<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub channel: &'a str,
}

impl<'a> IrcCommand<'a> {
    /// Construct an IRC command as a simple command with an associated channel.
    pub fn new(msg: &'a IrcPrivMsg, key: &'a str) -> Option<IrcCommand<'a>> {
        match Command::new(msg.txt.as_slice(), key) {
            Some(cmd) => {
                Some(IrcCommand {
                    name: cmd.name,
                    args: cmd.args,
                    channel: msg.channel.as_slice(),
                })
            },
            None => None,
        }
    }
}

/// A command without a channel. Otherwise analogous to IrcCommand.
#[deriving(Show)]
pub struct Command<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    /// Attempts to process the command into name and arguments. This may fail.
    pub fn new(s: &'a str, key: &'a str) -> Option<Command<'a>> {
        let s = s.trim();
        if s.len() > 0 && s.starts_with(key) {
            let split = space_split(s);
            let name = split[1];
            let mut args = Vec::new();
            args.push_all(split.slice_from(2));

            Some(Command {
                name: name,
                args: args,
            })
        }
        else {
            None
        }
    }
}
