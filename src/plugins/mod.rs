// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Module
//!
//! Collects and registers plugins. If a plugin isn't actually registered here,
//! it won't be included in the response handling.

use irc::{Irc};

pub use plugins::uptime::*;
pub use plugins::describe::*;
pub use plugins::excuse::*;
pub use plugins::officers::*;
pub use plugins::default::*;

mod uptime;
mod describe;
mod excuse;
mod officers;
mod default;


/// Registers all plugins
pub fn register(irc: &mut Irc) {
    irc.register_plugin(box Describe::new());
    irc.register_plugin(box Uptime::new());
    irc.register_plugin(box Excuse::new());
    irc.register_plugin(box Officers::new());
    irc.register_plugin(box Default::new());
}

