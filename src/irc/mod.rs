// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # IRC Mod
//!
//! Reexport everything in the IRC module.

#![macro_escape]

// We can reexport what we want to show from this module.
pub use irc::config::IrcConfig;
pub use irc::connection::{ConnectionEvent, ServerConnection};
pub use irc::msg::IrcMsg;
pub use irc::privmsg::IrcPrivMsg;
pub use irc::writer::IrcWriter;
pub use irc::info::BotInfo;
pub use irc::command::{IrcCommand, Command};
pub use irc::irc::Irc;
pub use irc::plugin::{Plugin, Handler};
pub use irc::config::JsonConfig;

mod config;
mod connection;
mod writer;
mod msg;
mod privmsg;
mod info;
mod command;
mod data;
mod irc;
mod plugin;
