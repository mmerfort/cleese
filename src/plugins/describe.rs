extern crate time;

use irc::{IrcWriter, IrcCommand, BotInfo, Plugin};

pub struct Describe;

impl Describe {
    pub fn new() -> Describe { Describe }
}

impl Plugin for Describe {
    fn cmd(&mut self, cmd: &IrcCommand, writer: &IrcWriter, _info: &BotInfo) {
        match cmd.name {
			"describe" => {
			    let msg = format!("{}", _info.descr);
			    writer.msg(cmd.channel.as_slice(), msg.as_slice());
			}
            _ => {}
        }
    }
}
