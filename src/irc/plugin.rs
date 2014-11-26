// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

use irc::{IrcWriter, IrcCommand, BotInfo, IrcPrivMsg};

pub trait Plugin {
    fn privmsg(&mut self,
               _msg:    &IrcPrivMsg,
               _writer: &IrcWriter,
               _info:   &BotInfo) {}

        fn cmd(&mut self,
               _cmd:    &IrcCommand,
               _writer: &IrcWriter,
               _info:   &BotInfo) {}

       fn help(&self) -> &'static str;
       fn name(&self) -> &'static str;
}

