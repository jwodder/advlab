use phf::{phf_map, Map};
use thiserror::Error;
use unicase::UniCase;

static VOCABULARY: Map<UniCase<&'static str>, Command> = phf_map! {
    UniCase::ascii("N") => Command::Motion(Motion::North),
    UniCase::ascii("NORTH") => Command::Motion(Motion::North),
    UniCase::ascii("S") => Command::Motion(Motion::South),
    UniCase::ascii("SOUTH") => Command::Motion(Motion::South),
    UniCase::ascii("E") => Command::Motion(Motion::East),
    UniCase::ascii("EAST") => Command::Motion(Motion::East),
    UniCase::ascii("W") => Command::Motion(Motion::West),
    UniCase::ascii("WEST") => Command::Motion(Motion::West),
    UniCase::ascii("EXAMINE") => Command::Examine,
    UniCase::ascii("LOOK") => Command::Examine,
    UniCase::ascii("DESCRIBE") => Command::Examine,
    UniCase::ascii("BACK") => Command::Back,
    UniCase::ascii("RETURN") => Command::Back,
    UniCase::ascii("RETREAT") => Command::Back,
    UniCase::ascii("QUIT") => Command::Quit,
    UniCase::ascii("EXIT") => Command::Quit,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Motion {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Command {
    Motion(Motion),
    Examine,
    Back,
    Quit,
    Nop,
}

impl std::str::FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Command, CommandError> {
        let s = s.trim();
        if s.is_empty() {
            Ok(Command::Nop)
        } else {
            VOCABULARY
                .get(&UniCase::ascii(s))
                .copied()
                .ok_or(CommandError)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("I don't know what that means.")]
pub(crate) struct CommandError;
