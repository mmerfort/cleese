// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Default
//!
//! Run this if a command was encountered but nothing matched it, and tell the
//! user how to access the help documentation.

use irc::{IrcPrivMsg, IrcWriter, IrcCommand, BotInfo, Plugin};


/// This struct only has the minimum number of fields for a plugin.
pub struct Describe {
    description: &'static str,
    name: &'static str
}


impl Describe {
    /// Just returns an instance of the unit struct.
    pub fn new() -> Describe {
        Describe {
            description: "Get information on accessing the help docs",
            name: "Default"
        }
    }
}

impl Plugin for Describe {
    /// Respond to private messages.
    ///
    /// Called by the plugin subsystem when a private message is received. It
    /// currently does nothing.
    fn privmsg(&mut self, _: &IrcPrivMsg, _: &IrcWriter, _: &BotInfo) {}

    /// Respond to received commands.
    ///
    /// Called by the plugin subsystem when a command is encountered. It only
    /// responds to the command "describe". Otherwise it does nothing.
    fn cmd(&mut self, cmd: &IrcCommand, writer: &IrcWriter, info: &BotInfo) {
        let msg = "Type `/msg cleese help` to see my list of commands.";
        writer.msg(cmd.channel.as_slice(), msg);
    }

    /// Return the plugin description.
    fn help(&self) -> &'static str { self.description }

    /// Return the plugin name.
    fn name(&self) -> &'static str { self.name }
}

