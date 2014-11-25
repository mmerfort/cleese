use core::fmt::{Show, Formatter, Result};
use irc::msg::*;

// A privmsg sent from the server.
pub struct IrcPrivMsg {
    pub orig: String,
    pub sender_nick: String,
    pub sender_info: String,
    pub channel: String,
    pub txt: String,
}

impl IrcPrivMsg {
    pub fn new(msg: &IrcMsg) -> Option<IrcPrivMsg> {
        if msg.code.as_slice() == "PRIVMSG" {
            match (msg.match_sender(), msg.match_message()) {
                (Some((nick, info)), Some((channel, txt))) =>
                    Some(IrcPrivMsg {
                        orig: msg.orig.clone(),
                        sender_nick: nick,
                        sender_info: info,
                        channel: channel,
                        txt: txt,
                    }),
                _ => None,
            }
        }
        else {
            None
        }
    }
}

impl Show for IrcPrivMsg {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "sender: {} ({}) channel: {} msg: {}",
               self.sender_nick, self.sender_info, self.channel, self.txt)
    }
}
