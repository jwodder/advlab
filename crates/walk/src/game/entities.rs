#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Entity {
    Banquet,
    Books,
    Cushions,
    Globe,
    Mural,
    Painting,
    PenguinPhoto,
    SecretPlans,
    TicTacToe,
}

impl Entity {
    // Text displayed for the entity when describing the containing room
    pub(crate) fn describe(&self) -> &str {
        match self {
            Entity::Banquet => "A banquet was set here, but someone has already eaten everything.",
            Entity::Books => "The walls are lined with shelves packed with books.",
            Entity::Cushions => {
                "There are numerous comfortable chairs here, and the floor is covered in cushions."
            }
            Entity::Globe => "A globe stands in the middle of the room.",
            Entity::Mural => "A mural of the rising sun decorates the wall.",
            Entity::Painting => "A painting of a full moon rests on an easel.",
            Entity::PenguinPhoto => "A photograph of a penguin couple hangs on the wall.",
            Entity::SecretPlans => "Secret plans for more games are scattered about!",
            Entity::TicTacToe => "There is a tick-tac-toe grid carved into the floor.",
        }
    }

    pub(crate) fn examine(&self) -> &str {
        match self {
            Entity::Banquet => "Judging by the crumbs, the meal was chicken nuggets.",
            Entity::Books => {
                "You've never heard of any of these titles before, but they all sound interesting!"
            }
            Entity::Cushions => {
                "The longer you stare at the cushioning, the more you want to just collapse into it."
            }
            Entity::Globe => "Wait, that's not Earth.  Where am I?",
            Entity::Mural => {
                "I don't know much about art, but it certainly looks fancy.  I think it's Art Nouveau?  Art Deco?  Something like that."
            }
            Entity::Painting => {
                "There is nothing to describe, except the moon, still bright against the worrying sky."
            }
            Entity::PenguinPhoto => "The penguins are grumpy-looking but are clearly in love.",
            Entity::SecretPlans => {
                "The plans are all written in code.  You can't make heads or tails of them."
            }
            Entity::TicTacToe => "X and O are locked in a dead heat.",
        }
    }

    pub(crate) fn read(&self) -> Option<&'static str> {
        match self {
            Entity::Books => Some("You sit and read for a while."),
            Entity::SecretPlans => {
                Some("The plans are all written in code.  You can't make heads or tails of them.")
            }
            _ => None,
        }
    }
}
