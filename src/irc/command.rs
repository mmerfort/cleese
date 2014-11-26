// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

use irc::privmsg::*;
use util::*;

// Command through irc.
#[deriving(Show)]
pub struct IrcCommand<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub channel: &'a str,
}

impl<'a> IrcCommand<'a> {
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

#[deriving(Show)]
pub struct Command<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Command<'a> {
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
