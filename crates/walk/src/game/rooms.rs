use super::vocab::Motion;

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
            Room::NorthWest => "You are in the north-west room.", // TODO: Expand
            Room::North => "You are in the north room.  It is very cold.",
            Room::NorthEast => "You are in the north-east room.", // TODO: Expand
            Room::West => "You are in the west room.  A mural of a full moon decorates the wall.",
            Room::Center => "You are in the center room.  There is an empty tick-tack-toe grid carved onto the floor.",
            Room::East => "You are in the east room.  A mural of the rising sun decorates the wall.",
            Room::SouthWest => "You are in the south-west room.", // TODO: Expand
            Room::South => "You are in the south room.  A photograph of a penguin couple hangs on the wall.",
            Room::SouthEast => "You are in the south-east room.", // TODO: Expand
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

pub(crate) const TRAVEL_TABLE: [((Room, Motion), Room); 24] = [
    ((Room::NorthWest, Motion::East), Room::North),
    ((Room::NorthWest, Motion::South), Room::West),
    ((Room::North, Motion::West), Room::NorthWest),
    ((Room::North, Motion::South), Room::Center),
    ((Room::North, Motion::East), Room::NorthEast),
    ((Room::NorthEast, Motion::West), Room::North),
    ((Room::NorthEast, Motion::South), Room::East),
    ((Room::West, Motion::North), Room::NorthWest),
    ((Room::West, Motion::East), Room::Center),
    ((Room::West, Motion::South), Room::SouthWest),
    ((Room::Center, Motion::North), Room::North),
    ((Room::Center, Motion::East), Room::East),
    ((Room::Center, Motion::West), Room::West),
    ((Room::Center, Motion::South), Room::South),
    ((Room::East, Motion::North), Room::NorthEast),
    ((Room::East, Motion::West), Room::Center),
    ((Room::East, Motion::South), Room::SouthEast),
    ((Room::SouthWest, Motion::East), Room::South),
    ((Room::SouthWest, Motion::North), Room::West),
    ((Room::South, Motion::West), Room::SouthWest),
    ((Room::South, Motion::North), Room::Center),
    ((Room::South, Motion::East), Room::SouthEast),
    ((Room::SouthEast, Motion::West), Room::South),
    ((Room::SouthEast, Motion::North), Room::East),
];
