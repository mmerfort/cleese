// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

//! # Officers
//!
//! Shares the list of current officers, along with their IRC names and degree
//! programs.

use std::fmt;
use irc::{IrcWriter, IrcCommand, BotInfo, Plugin};

enum Position {
    President,
    VicePresident,
    Treasurer,
    Secretary,
    InformationOfficer,
    ActivitiesDirector,
    Webmaster
}

impl fmt::Show for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Position::President          => write!(f, "President"),
            Position::VicePresident      => write!(f, "Vice President"),
            Position::Treasurer          => write!(f, "Treasurer"),
            Position::Secretary          => write!(f, "Secretary"),
            Position::InformationOfficer => write!(f, "Information Officer"),
            Position::ActivitiesDirector => write!(f, "Activities Director"),
            Position::Webmaster          => write!(f, "Webmaster")
        }
    }
}

enum Program {
    CS,
    CE,
    Interdisciplinary,
    WebProgramming,
    SystemAdministrator,
    GameDevelopment,
    GraphicsProgramming,
    Bioinformatics,
    CSMasters,
    CSMinor,
    CSCertificate
}

impl fmt::Show for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Program::CS                  => write!(f, "BS in Computer Science"),
            Program::CE                  => write!(f, "BS in Computer Engineering"),
            Program::Interdisciplinary   => write!(f, "BA in Computer Systems, Interdisciplinary Option"),
            Program::WebProgramming      => write!(f, "BA in Computer Systems, Web Programming Option"),
            Program::SystemAdministrator => write!(f, "BA in Computer Systems, System Administrator Option"),
            Program::GameDevelopment     => write!(f, "BA in Computer Systems, Game Development Option"),
            Program::GraphicsProgramming => write!(f, "BA in Computer Systems, Graphics Programming Option"),
            Program::Bioinformatics      => write!(f, "BS in Bioinformatics"),
            Program::CSMasters           => write!(f, "Masters in Computer Science"),
            Program::CSMinor             => write!(f, "Minor in Computer Science"),
            Program::CSCertificate       => write!(f, "Certificate in Computer Science"),
        }
    }
}

struct Officer {
    name: &'static str,
    position: Position,
    program: Program,
    irc_nick: Option<&'static str>
}

impl fmt::Show for Officer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let irc_nick = match self.irc_nick {
            Some(x) => x,
            None => "<None>"
        };
        write!(f, "({3}) {0}, {1}. {2}",
               self.name, self.position, self.program, irc_nick)
    }
}

pub struct Officers {
    officers: Vec<Officer>
}

impl Officers {
    pub fn new() -> Officers {
        Officers {
            officers: vec![
                Officer {
                    name: "Dylan Allbee",
                    position: Position::President,
                    program: Program::CE,
                    irc_nick: Some("dallbee"),
                },
                Officer {
                    name: "Andrew Brinker",
                    position: Position::VicePresident,
                    program: Program::CS,
                    irc_nick: Some("Hemamorphy"),
                },
                Officer {
                    name: "Beverly Abadines",
                    position: Position::Treasurer,
                    program: Program::CE,
                    irc_nick: None,
                },
                Officer {
                    name: "Abigail Legg",
                    position: Position::Secretary,
                    program: Program::CSMasters,
                    irc_nick: None,
                },
                Officer {
                    name: "Ammar Alsibai",
                    position: Position::ActivitiesDirector,
                    program: Program::CE,
                    irc_nick: Some("Pyrot1c"),
                },
                Officer {
                    name: "Anthony Sterrett",
                    position: Position::InformationOfficer,
                    program: Program::CS,
                    irc_nick: Some("Arandur"),
                },
                Officer {
                    name: "Mike Korcha",
                    position: Position::Webmaster,
                    program: Program::CSMasters,
                    irc_nick: Some("korcha"),
                },
            ]
        }
    }

    fn officers(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Show for Officers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for officer in self.officers.iter() {
            result.push_str(format!("{}\n", officer).as_slice());
        }
        write!(f, "{}", result)
    }
}

impl Plugin for Officers {
    fn cmd(&mut self, cmd: &IrcCommand, writer: &IrcWriter, _info: &BotInfo) {
        match cmd.name {
            "officers" => {
                writer.msg(cmd.channel.as_slice(), self.officers().as_slice());
            }
            _ => {}
        }
    }
}
