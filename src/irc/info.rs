// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Info
//!
//! This is where the information about the bot is maintained. The bot has a
//! nick, description, vector of channels, and a command prefix. The command
//! prefix is how commands are identified by the bot.

use irc::config::IrcConfig;


/// Information about our bot.
pub struct BotInfo<'a> {
    pub nick: &'a str,
    pub descr: &'a str,
    pub channels: Vec<&'a str>,
    pub cmd_prefix: &'a str,
}

impl<'a> BotInfo<'a> {
    /// Construct a new BotInfo.
    pub fn new(conf: &IrcConfig<'a>) -> BotInfo<'a> {
        BotInfo {
            nick: conf.nick,
            descr: conf.descr,
            channels: conf.channels.clone(),
            cmd_prefix: conf.cmd_prefix,
        }
    }
}

