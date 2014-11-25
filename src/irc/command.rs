//use core::fmt::{Show, Formatter, Result};

use irc::privmsg::*;
use util::*;

// Command through irc.
#[deriving(Show)]
pub struct IrcCommand<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub channel: &'a str,
}

// TODO something like this?
impl<'a> IrcCommand<'a> {
    pub fn new(msg: &'a IrcPrivMsg, key: char) -> Option<IrcCommand<'a>> {
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

#[deriving(Show)]
pub struct Command<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn new(s: &'a str, key: char) -> Option<Command<'a>> {
        let s = s.trim();
        if s.len() > 0 && s.char_at(0) == key {
            let split = space_split(s);
            let name = split[0].slice_from(1);
            let mut args = Vec::new();
            args.push_all(split.slice_from(1));

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
