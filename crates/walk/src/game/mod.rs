mod rooms;
mod vocab;
use self::rooms::{Room, TRAVEL_TABLE};
use self::vocab::{Command, Motion};
use ifcore::{GameBuilder, GameEngine, Output};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Builder;

impl GameBuilder for Builder {
    type Engine = Game;

    fn start(self) -> Output<Game> {
        let travel = HashMap::from(TRAVEL_TABLE);
        let location = Room::Center;
        let visited = HashSet::new();
        let mut game = Game {
            travel,
            location,
            prev_location: None,
            visited,
        };
        let text = game.show_location();
        Output::Continue { game, text }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Game {
    travel: HashMap<(Room, Motion), Room>,
    location: Room,
    prev_location: Option<Room>,
    visited: HashSet<Room>,
}

impl Game {
    fn show_location(&mut self) -> String {
        let first_time = self.visited.insert(self.location);
        if first_time {
            self.location.long_description().to_owned()
        } else {
            self.location.short_description().to_owned()
        }
    }

    fn move_to(&mut self, room: Room) -> String {
        self.prev_location = Some(self.location);
        self.location = room;
        self.show_location()
    }
}

impl GameEngine for Game {
    fn handle_input(mut self, input: &str) -> Output<Self> {
        let text = match input.parse::<Command>() {
            Ok(Command::Motion(m)) => {
                if let Some(room) = self.travel.get(&(self.location, m)).copied() {
                    self.move_to(room)
                } else {
                    String::from("There's no way to go in that direction.")
                }
            }
            Ok(Command::Examine) => self.location.long_description().to_owned(),
            Ok(Command::Back) => {
                if let Some(prev) = self.prev_location {
                    self.move_to(prev)
                } else {
                    String::from("You weren't anywhere else before here.")
                }
            }
            Ok(Command::Quit) => {
                return Output::Goodbye {
                    text: String::from("Be seeing you..."),
                }
            }
            Err(e) => e.to_string(),
        };
        Output::Continue { game: self, text }
    }
}

#[cfg(test)]
mod tests;
