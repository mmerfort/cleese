use std::rand;
use irc::{IrcWriter, IrcCommand, BotInfo, Plugin};

pub struct Excuse {
    excuses: Vec<&'static str>
}

impl Excuse {
    pub fn new() -> Excuse {
        Excuse {
            excuses: vec![
                "Our code quality is no worse than anyone else in the industry.",
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
                "The original specification contained conflicting requirements.",
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
                "I was told to stop working on that when something important came up.",
                "I thought you signed off on that.",
                "That wouldn't be economically feasible.",
                "I didn't create that part of the program.",
                "It probably won't happen again.",
                "Actually, that's a feature.",
                "My time was split in a way that meant I couldn't do either project properly.",
                "I have too many other high priority things to do right now.",
                "Our internet connection must not be working.",
                "The accounting department must have cancelled that subscription.",
                "It's always been like that.",
                "What did you type in wrong to get it to crash?",
                "What did I tell you about using parts of the system you don't understand?",
                "It was working in my head.",
                "I thought I finished that.",
                "I must have been stress testing our production server.",
                "The request must have dropped some packets."
            ]
        }
    }

    fn excuse(&self) -> String {
        let choice = rand::random::<uint>() % (self.excuses.len() as uint);
        format!("{}", self.excuses[choice])
    }
}

impl Plugin for Excuse {
    fn cmd(&mut self, cmd: &IrcCommand, writer: &IrcWriter, _info: &BotInfo) {
        match cmd.name {
            "excuse" => {
                writer.msg(cmd.channel.as_slice(), self.excuse().as_slice());
            },
            _ => {},
        }
    }
}


