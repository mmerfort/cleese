// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

use irc::{IrcWriter, IrcCommand, BotInfo, IrcPrivMsg};

pub trait Plugin {
    fn privmsg(&mut self, msg: &IrcPrivMsg, writer: &IrcWriter, info: &BotInfo);
    fn cmd(&mut self, cmd: &IrcCommand, writer: &IrcWriter, info: &BotInfo);
    fn help(&self) -> &'static str;
    fn name(&self) -> &'static str;
}

