use irc::{ Irc };

pub use plugins::basic::*;
pub use plugins::excuse::*;

mod basic;
mod excuse;

pub fn register(irc: &mut Irc) {
    irc.register_plugin(box Basic::new());
    irc.register_plugin(box Excuse::new());
}

