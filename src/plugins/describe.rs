// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Describe
//!
//! This plugin just shares the bot description taken from the config file.

use irc::{IrcWriter, IrcCommand, BotInfo, Plugin};


/// A unit struct, defined here only to be consistent with the Plugin interface.
pub struct Describe;


impl Describe {
    /// Just returns an instance of the unit struct.
    pub fn new() -> Describe { Describe }
}

impl Plugin for Describe {
    /// Respond to received commands.
    ///
    /// Called by the plugin subsystem when a command is encountered. It only
    /// responds to the command "describe". Otherwise it does nothing.
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

