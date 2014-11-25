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
}

