// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Excuse
//!
//! A fun little plugin that gets a random excuse (shamelessly stolen from
//! [developerexcuses.com](http://developerexcuses.com/)) and shares it in the
//! IRC chat.

use std::rand;
use irc::{IrcPrivMsg, IrcWriter, IrcCommand, BotInfo, Plugin, Handler};


/// Contains a vector of excuses, defined in the constructor.
pub struct Excuse {
    excuses: Vec<&'static str>,
    description: &'static str,
    name: &'static str
}

impl Excuse {
    /// Construct the vector of excuses, currently encoded as string slices
    /// directly in the source.
    pub fn new() -> Excuse {
        Excuse {
            excuses: vec![
                "I thought you signed off on that.",
                "Where were you when the program blew up?",
                "That feature was slated for phase two.",
                "That feature would be outside the scope.",
                "It must be a hardware problem.",
                "That isn't covered by my job description.",
                "It's never shown unexpected behavior like this before.",
                "There must be something strange in your data.",
                "Well, that's a first.",
                "I haven't touched that code in weeks.",
                "Oh, you said you DIDN'T want that to happen?",
                "That's already fixed. It just hasn't taken effect yet.",
                "I couldn't find any library that can even do that.",
                "I usually get a notification when that happens",
                "Oh, that was just a temporary fix.",
                "It's never done that before.",
                "It's a compatibility issue.",
                "I didn't anticipate that I would make any errors.",
                "I did a quick fix last time but it broke when we rebooted.",
                "Everything looks fine on my end.",
                "That error means it was successful.",
                "The marketing department made us put that there.",
                "I forgot to commit the code that fixes that.",
                "Oh, that was only supposed to be a placeholder.",
                "I haven't had a chance to run that code yet.",
                "You must have done something wrong.",
                "Well, at least it displays a very pretty error.",
                "That wasn't in the original specification.",
                "I haven't had any experience with that before.",
                "That's the fault of the graphic designer.",
                "I'll have to fix that at a later date.",
                "I told you yesterday it would be done by the end of today.",
                "I haven't been able to reproduce that.",
                "It's just some unlucky coincidence.",
                "I thought you signed off on that.",
                "That wouldn't be economically feasible.",
                "I didn't create that part of the program.",
                "It probably won't happen again.",
                "Actually, that's a feature.",
                "I have too many other high priority things to do right now.",
                "Our internet connection must not be working.",
                "It's always been like that.",
                "What did you type in wrong to get it to crash?",
                "It was working in my head.",
                "I thought I finished that.",
                "I must have been stress testing our production server.",
                "The request must have dropped some packets."
            ],
            description: "Get an excuse from developerexcuses.com",
            name: "excuse"
        }
    }

    /// Select a random excuse from the vector of excuses.
    fn excuse(&self) -> String {
        let choice = rand::random::<uint>() % (self.excuses.len() as uint);
        format!("{}", self.excuses[choice])
    }
}

impl Plugin for Excuse {
    /// Respond to private messages.
    ///
    /// Called by the plugin subsystem when a private message is received. It
    /// currently does nothing.
    fn privmsg(&mut self, _: &IrcPrivMsg,
               _: &IrcWriter, _: &BotInfo) -> Handler {
        Handler::Passed
    }

    /// Respond to received commands.
    ///
    /// Called by the plugin subsystem when a command is encountered. It only
    /// responds to the command "excuse". Otherwise it does nothing.
    fn cmd(&mut self, cmd: &IrcCommand,
           writer: &IrcWriter, _info: &BotInfo) -> Handler {
        match cmd.name {
            "excuse" => {
                writer.msg(cmd.channel.as_slice(), self.excuse().as_slice());
                Handler::Accepted
            },
            _ => { Handler::Passed }
        }
    }

    /// Return the plugin description.
    fn help(&self) -> &'static str { self.description }

    /// Return the plugin name.
    fn name(&self) -> &'static str { self.name }
}


