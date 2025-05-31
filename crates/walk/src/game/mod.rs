mod data;
mod entities;
mod rooms;
mod vocab;
use self::entities::Entity;
use self::rooms::Room;
use self::vocab::{Command, Motion};
use ifcore::{GameBuilder, GameEngine, Output};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Builder;

impl GameBuilder for Builder {
    type Engine = Game;

    fn start(self) -> Output<Game> {
        let mut game = Game::new();
        let text = game.show_location(None);
        Output::Continue { game, text }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Game {
    travel: HashMap<(Room, Motion), Room>,
    location: Room,
    prev_location: Option<Room>,
    visited: HashSet<Room>,
    fixed: HashMap<Entity, Room>,
}

impl Game {
    fn new() -> Game {
        let travel = HashMap::from(data::TRAVEL_TABLE);
        let location = Room::Center;
        let visited = HashSet::new();
        let fixed = HashMap::from(data::FIXED_ENTITIES);
        Game {
            travel,
            location,
            prev_location: None,
            visited,
            fixed,
        }
    }

    fn show_location(&mut self, long: Option<bool>) -> String {
        let long = long.unwrap_or_else(|| self.visited.insert(self.location));
        let mut s = if long {
            self.location.long_description().to_owned()
        } else {
            self.location.short_description().to_owned()
        };
        for (&en, &rm) in &self.fixed {
            if rm == self.location {
                s.push('\n');
                s.push('\n');
                s.push_str(en.describe());
            }
        }
        s
    }

    fn move_to(&mut self, room: Room) -> String {
        self.prev_location = Some(self.location);
        self.location = room;
        self.show_location(None)
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
            Ok(Command::Examine(None)) => self.show_location(Some(true)),
            Ok(Command::Examine(Some(en))) => {
                if self.fixed.get(&en) == Some(&self.location) {
                    en.examine().to_owned()
                } else {
                    String::from("That isn't here.")
                }
            }
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
            Ok(Command::Nop) => String::new(),
            Err(e) => e.to_string(),
        };
        Output::Continue { game: self, text }
    }
}

#[cfg(test)]
mod tests;
