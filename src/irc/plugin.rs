// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

use irc::{IrcWriter, IrcCommand, BotInfo, IrcPrivMsg};

pub enum HandleResult {
    Accepted,
    Passed
}

/// Trait defining all cleese plugins.
///
/// This is the most important trait in the entire system. Plugins are how
/// cleese defines what commands to accept, and are the basis for all of its
/// user-facing functionality.
///
/// All plugins in cleese have to be able to respond to private messages and
/// commands (although they can simply do nothing if they wish), as well as
/// provide basic information about themselves. To make a plugin, simply define
/// a struct implementing this trait, and then register it in
/// `src/plugins/mod.rs`.
pub trait Plugin {
    /// Respond to private messages.
    fn privmsg(&mut self, msg: &IrcPrivMsg,
               writer: &IrcWriter, info: &BotInfo) -> HandleResult;

    /// Respond to commands.
    fn cmd(&mut self, cmd: &IrcCommand,
           writer: &IrcWriter, info: &BotInfo) -> HandleResult;

    /// Provide help text.
    fn help(&self) -> &'static str;

    /// Provide plugin name.
    fn name(&self) -> &'static str;
}

