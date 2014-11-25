use irc::{Irc};

pub use plugins::uptime::*;
pub use plugins::describe::*;
pub use plugins::excuse::*;
pub use plugins::officers::*;

mod uptime;
mod describe;
mod excuse;
mod officers;

pub fn register(irc: &mut Irc) {
    irc.register_plugin(box Uptime::new());
    irc.register_plugin(box Describe::new());
    irc.register_plugin(box Excuse::new());
    irc.register_plugin(box Officers::new());
}

