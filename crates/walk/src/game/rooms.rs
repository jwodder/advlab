#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Room {
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl Room {
    pub(crate) fn long_description(&self) -> &str {
        match self {
            Room::NorthWest => {
                "You are in the north-west room.  A delicious smell lingers in the air."
            }
            Room::North => "You are in the north room.  It is very cold here.",
            Room::NorthEast => "You are in the north-east room.",
            Room::West => "You are in the west room.  The lights are turned down low.",
            Room::Center => {
                "You are in the center room.  Doors lead out in all cardinal directions."
            }
            Room::East => "You are in the east room.  It gives off a bright & cheery air.",
            Room::SouthWest => "You are in the south-west room.",
            Room::South => "You are in the south room.  Antarctic memorabilia are scattered about.",
            Room::SouthEast => "You are in the south-east room.",
        }
    }

    pub(crate) fn short_description(&self) -> &str {
        match self {
            Room::NorthWest => "You are in the north-west room.",
            Room::North => "You are in the north room.",
            Room::NorthEast => "You are in the north-east room.",
            Room::West => "You are in the west room.",
            Room::Center => "You are in the center room.",
            Room::East => "You are in the east room.",
            Room::SouthWest => "You are in the south-west room.",
            Room::South => "You are in the south room.",
            Room::SouthEast => "You are in the south-east room.",
        }
    }
}
