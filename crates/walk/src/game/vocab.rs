use super::entities::Entity;
use thiserror::Error;
use unicase::UniCase;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Word {
    Motion(Motion),
    Action(Action),
    Entity(Entity),
}

impl std::str::FromStr for Word {
    type Err = WordError;

    fn from_str(s: &str) -> Result<Word, WordError> {
        super::data::VOCABULARY
            .get(&UniCase::ascii(s))
            .copied()
            .ok_or_else(|| WordError(s.to_owned()))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Motion {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Action {
    Examine,
    Read,
    Back,
    Quit,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Command {
    Motion(Motion),
    Examine(Option<Entity>),
    Read(Option<Entity>),
    Back,
    Quit,
    Nop,
}

impl From<Action> for Command {
    fn from(value: Action) -> Command {
        match value {
            Action::Examine => Command::Examine(None),
            Action::Read => Command::Read(None),
            Action::Back => Command::Back,
            Action::Quit => Command::Quit,
        }
    }
}

impl std::str::FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Command, CommandError> {
        let words = s
            .split_whitespace()
            .map(str::parse::<Word>)
            .collect::<Result<Vec<_>, _>>()?;
        match words.as_slice() {
            [Word::Motion(m)] => Ok(Command::Motion(*m)),
            [Word::Action(act)] => Ok(Command::from(*act)),
            [Word::Action(Action::Examine), Word::Entity(en)] => Ok(Command::Examine(Some(*en))),
            [Word::Action(Action::Read), Word::Entity(en)] => Ok(Command::Read(Some(*en))),
            [] => Ok(Command::Nop),
            _ => Err(CommandError::BadGrammar),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum CommandError {
    #[error("I know what those words mean, but that sentence makes no sense.")]
    BadGrammar,
    #[error(transparent)]
    Word(#[from] WordError),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("I don't know what {0:?} means.")]
pub(crate) struct WordError(String);
