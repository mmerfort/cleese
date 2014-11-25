use irc::{Irc};

pub use plugins::basic::*;
pub use plugins::excuse::*;
pub use plugins::officers::*;

mod basic;
mod excuse;
mod officers;

pub fn register(irc: &mut Irc) {
    irc.register_plugin(box Basic::new());
    irc.register_plugin(box Excuse::new());
    irc.register_plugin(box Officers::new());
}

